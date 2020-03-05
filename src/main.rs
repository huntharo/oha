use clap::Clap;
use futures::prelude::*;
use url::Url;

mod monitor;
mod printer;
mod work;

struct ParseDuration(std::time::Duration);

impl std::str::FromStr for ParseDuration {
    type Err = parse_duration::parse::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_duration::parse(s).map(ParseDuration)
    }
}

#[derive(Clap)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!())]
struct Opts {
    #[clap(help = "Target URL.")]
    url: String,
    #[clap(help = "Number of requests.", short = "n", default_value = "200")]
    n_requests: usize,
    #[clap(help = "Number of workers.", short = "c", default_value = "50")]
    n_workers: usize,
    #[clap(help = "Duration.\nExamples: -z 10s -z 3m.", short = "z")]
    duration: Option<ParseDuration>,
    #[clap(help = "Query per second limit.", short = "q")]
    query_per_second: Option<usize>,
    #[clap(help = "No realtime tui", long = "no-tui")]
    no_tui: bool,
    #[clap(help = "Frame per second for tui.", default_value = "8", long = "fps")]
    fps: usize,
}

#[derive(Debug, Clone)]
pub struct RequestResult {
    start: std::time::Instant,
    end: std::time::Instant,
    status: reqwest::StatusCode,
    len_bytes: usize,
}

impl RequestResult {
    pub fn duration(&self) -> std::time::Duration {
        self.end - self.start
    }
}

async fn request(
    client: reqwest::Client,
    url: Url,
    reporter: tokio::sync::mpsc::UnboundedSender<anyhow::Result<RequestResult>>,
) -> Result<(), tokio::sync::mpsc::error::SendError<anyhow::Result<RequestResult>>> {
    let result = async move {
        let start = std::time::Instant::now();
        let resp = client.get(url.clone()).send().await?;
        let status = resp.status();
        let len_bytes = resp.bytes().await?.len();
        let end = std::time::Instant::now();
        Ok::<_, anyhow::Error>(RequestResult {
            start,
            end,
            status,
            len_bytes,
        })
    }
    .await;
    reporter.send(result)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut opts: Opts = Opts::parse();
    let url = Url::parse(opts.url.as_str())?;
    let client = reqwest::Client::new();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let start = std::time::Instant::now();

    let data_collector = if opts.no_tui {
        tokio::spawn(async move {
            let mut all: Vec<anyhow::Result<RequestResult>> = Vec::new();
            loop {
                tokio::select! {
                    report = rx.recv() => {
                        if let Some(report) = report {
                            all.push(report);
                        } else {
                            break;
                        }
                    }
                    Ok(()) = tokio::signal::ctrl_c() => {
                        printer::print(&all, start.elapsed());
                        std::process::exit(0);
                    }
                }
            }
            all
        })
        .boxed()
    } else {
        use std::io;

        use termion::input::MouseTerminal;
        use termion::raw::IntoRawMode;
        use termion::screen::AlternateScreen;
        use tui::backend::TermionBackend;
        use tui::Terminal;

        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        tokio::spawn(
            monitor::Monitor {
                terminal,
                end_line: opts
                    .duration
                    .as_ref()
                    .map(|d| monitor::EndLine::Duration(d.0))
                    .unwrap_or(monitor::EndLine::NumQuery(opts.n_requests)),
                report_receiver: rx,
                start,
                fps: opts.fps,
            }
            .monitor(),
        )
        .boxed()
    };

    if let Some(ParseDuration(duration)) = opts.duration.take() {
        if let Some(qps) = opts.query_per_second.take() {
            work::work_duration_with_qps(
                || request(client.clone(), url.clone(), tx.clone()),
                qps,
                duration,
                opts.n_workers,
            )
            .await
        } else {
            work::work_duration(
                || request(client.clone(), url.clone(), tx.clone()),
                duration,
                opts.n_workers,
            )
            .await
        }
    } else {
        if let Some(qps) = opts.query_per_second.take() {
            work::work_with_qps(
                || request(client.clone(), url.clone(), tx.clone()),
                qps,
                opts.n_requests,
                opts.n_workers,
            )
            .await
        } else {
            work::work(
                || request(client.clone(), url.clone(), tx.clone()),
                opts.n_requests,
                opts.n_workers,
            )
            .await
        }
    };
    std::mem::drop(tx);

    let res: Vec<_> = data_collector.await?;
    let duration = std::time::Instant::now() - start;

    printer::print(&res, duration);

    Ok(())
}

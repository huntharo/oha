Install pprof:
https://github.com/google/pprof

Rust Memory Profiling:
https://www.polarsignals.com/blog/posts/2023/12/20/rust-memory-profiling

```sh
cargo run -- -c 100 -z 5m --insecure https://host.docker.internal:3001/ping

cargo run -- -c 20 -p 20 -z 5m --http2 --insecure https://host.docker.internal:3001/ping

curl localhost:3002/debug/pprof/heap > heap.pb.gz
pprof -http=:8080 heap.pb.gz
```



Exception
```
Error: hyper::Error(Io, Custom { kind: UnexpectedEof, error: "peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof" })────────────────────────────┘
                                                                            Error: hyper::Error(Io, Custom { kind: UnexpectedEof, error: "peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof" })
                                                                    Error: hyper::Error(Io, Custom { kind: UnexpectedEof, error: "peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof" })
                                                            Error: hyper::Error(Io, Custom { kind: UnexpectedEof, error: "peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof" })
                                                    Error: hyper::Error(Io, Custom { kind: UnexpectedEof, error: "peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof" })
                                            Error: hyper::Error(Io, Custom { kind: UnexpectedEof, error: "peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof" })
```
use std::time::Duration;

#[cfg(target_os = "linux")]
pub(crate) fn thread_cpu_time() -> Option<Duration> {
    let mut timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let result = unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut timespec) };
    if result != 0 {
        return None;
    }

    Some(Duration::new(
        timespec.tv_sec as u64,
        timespec.tv_nsec as u32,
    ))
}

#[cfg(target_os = "macos")]
pub(crate) fn thread_cpu_time() -> Option<Duration> {
    let thread = unsafe { mach2::mach_init::mach_thread_self() };
    let mut info = unsafe { std::mem::zeroed::<libc::thread_basic_info>() };
    let mut count = libc::THREAD_BASIC_INFO_COUNT;
    let status = unsafe {
        libc::thread_info(
            thread,
            libc::THREAD_BASIC_INFO as u32,
            &mut info as *mut _ as libc::thread_info_t,
            &mut count,
        )
    };
    unsafe {
        mach2::mach_port::mach_port_deallocate(mach2::traps::mach_task_self(), thread);
    }
    if status != libc::KERN_SUCCESS {
        return None;
    }

    let user = Duration::new(
        info.user_time.seconds as u64,
        (info.user_time.microseconds as u32) * 1_000,
    );
    let system = Duration::new(
        info.system_time.seconds as u64,
        (info.system_time.microseconds as u32) * 1_000,
    );

    Some(user + system)
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub(crate) fn thread_cpu_time() -> Option<Duration> {
    None
}

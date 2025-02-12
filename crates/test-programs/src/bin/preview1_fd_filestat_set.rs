use std::{env, process, time::Duration};
use test_programs::preview1::{assert_fs_time_eq, open_scratch_directory, TestConfig};

unsafe fn test_fd_filestat_set(dir_fd: wasi::Fd) {
    let cfg = TestConfig::from_env();
    // Create a file in the scratch directory.
    let file_fd = wasi::path_open(
        dir_fd,
        0,
        "file",
        wasi::OFLAGS_CREAT,
        wasi::RIGHTS_FD_READ | wasi::RIGHTS_FD_WRITE,
        0,
        0,
    )
    .expect("failed to create file");
    assert!(
        file_fd > libc::STDERR_FILENO as wasi::Fd,
        "file descriptor range check",
    );

    // Check file size
    let stat = wasi::fd_filestat_get(file_fd).expect("failed filestat");
    assert_eq!(stat.size, 0, "file size should be 0");

    // Check fd_filestat_set_size
    wasi::fd_filestat_set_size(file_fd, 100).expect("fd_filestat_set_size");

    let stat = wasi::fd_filestat_get(file_fd).expect("failed filestat 2");
    assert_eq!(stat.size, 100, "file size should be 100");

    // Check fd_filestat_set_times
    let old_atim = Duration::from_nanos(stat.atim);
    let new_mtim = Duration::from_nanos(stat.mtim) - cfg.fs_time_precision() * 2;
    wasi::fd_filestat_set_times(
        file_fd,
        new_mtim.as_nanos() as u64,
        new_mtim.as_nanos() as u64,
        wasi::FSTFLAGS_MTIM,
    )
    .expect("fd_filestat_set_times");

    let stat = wasi::fd_filestat_get(file_fd).expect("failed filestat 3");
    assert_eq!(stat.size, 100, "file size should remain unchanged at 100");

    // Support accuracy up to at least 1ms
    assert_fs_time_eq!(
        Duration::from_nanos(stat.mtim),
        new_mtim,
        "mtim should change"
    );
    assert_fs_time_eq!(
        Duration::from_nanos(stat.atim),
        old_atim,
        "atim should not change"
    );

    // let status = wasi_fd_filestat_set_times(file_fd, new_mtim, new_mtim, wasi::FILESTAT_SET_MTIM | wasi::FILESTAT_SET_MTIM_NOW);
    // assert_eq!(status, wasi::EINVAL, "ATIM & ATIM_NOW can't both be set");

    wasi::fd_close(file_fd).expect("failed to close fd");
    wasi::path_unlink_file(dir_fd, "file").expect("failed to remove dir");
}
fn main() {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("usage: {} <scratch directory>", prog);
        process::exit(1);
    };

    // Open scratch directory
    let dir_fd = match open_scratch_directory(&arg) {
        Ok(dir_fd) => dir_fd,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };

    // Run the tests.
    unsafe { test_fd_filestat_set(dir_fd) }
}

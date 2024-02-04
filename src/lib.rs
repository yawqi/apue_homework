use nix::libc::{close, dup};

fn cleanup_fds(fds: Vec<i32>) {
    fds.into_iter().for_each(|fd| unsafe {
        close(fd);
    });
}

pub fn dup2_2(oldfd: i32, newfd: i32) -> i32 {
    let mut fds = vec![];
    if oldfd < 0 || newfd < 0 {
        return -1;
    }

    let startfd = unsafe { dup(oldfd) };
    if startfd < 0 {
        return -1;
    }
    fds.push(startfd);

    for _ in startfd + 1..newfd {
        let tmpfd = unsafe { dup(oldfd) };
        if tmpfd < 0 {
            cleanup_fds(fds);
            return -1;
        }
        fds.push(tmpfd);
    }

    unsafe { close(newfd) };
    let retfd = unsafe { dup(oldfd) };
    if retfd < 0 {
        return -1;
    }
    cleanup_fds(fds);
    newfd
}

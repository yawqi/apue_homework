use nix::libc::{lseek, open, read, remove, write, O_APPEND, O_CREAT, O_RDWR, O_TRUNC};
use std::ffi::c_void;
fn main() {
    let tmp_file = "tmp.txt";
    let f = unsafe {
        open(
            tmp_file.as_ptr() as *const i8,
            O_RDWR | O_APPEND | O_CREAT | O_TRUNC,
        )
    };

    if f < 0 {
        println!("Open file failed");
        stringerr();
        return;
    }

    let tail_content = "This is the tail!";
    let read_buf = [0u8; 100];
    let head_content = "Can this be head????";
    let mut ret = 0;
    unsafe {
        ret = write(
            f,
            tail_content.as_ptr() as *const c_void,
            tail_content.len(),
        );

        if ret != tail_content.len() as isize {
            println!("Write tail content failed");
            return;
        }

        ret = lseek(f, 0, 0) as isize;
        if ret == -1 {
            println!("lseek failed");
            return;
        }

        ret = read(f, read_buf.as_ptr() as *mut c_void, tail_content.len());
        if ret != tail_content.len() as isize {
            println!("Read tail content failed");
            return;
        }
        println!("read buf: {:?}", read_buf);

        ret = lseek(f, 0, 0) as isize;
        if ret == -1 {
            println!("lseek failed");
            return;
        }

        ret = write(
            f,
            head_content.as_ptr() as *const c_void,
            head_content.len(),
        );
        if ret == head_content.len() as isize {
            println!("write head content failed {}", ret);
            // return;
        }

        ret = lseek(f, 0, 0) as isize;
        if ret == -1 {
            println!("lseek failed");
            return;
        }
        ret = read(
            f,
            read_buf.as_ptr() as *mut c_void,
            head_content.len() + tail_content.len(),
        );
        if ret != head_content.len() as isize {
            println!("Read tail content failed");
            return;
        }
        println!("read buf: {:?}", read_buf);
    }

    unsafe {
        remove(tmp_file.as_ptr() as *const i8);
    }
}

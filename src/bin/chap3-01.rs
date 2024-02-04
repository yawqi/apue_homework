use apue::dup2_2;
use nix::unistd::write;
use std::process::exit;

fn main() {
    let mut new_stdout = dup2_2(1, 10);
    if new_stdout != 10 {
        println!("Wrong fd {new_stdout}");
        exit(-1);
    }

    let s = "Hello world from new stdout\n";
    write(new_stdout, s.as_bytes());

    new_stdout = dup2_2(1, 2);
    if new_stdout != 2 {
        println!("Wrong fd {new_stdout}");
        exit(-1);
    }

    let s = "Hello world from new stdout\n";
    write(new_stdout, s.as_bytes());
}

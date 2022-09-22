use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn learn() {
    // test_base_use();
    // test_pipes();
    test_wait();
}

// base use
fn test_base_use() {
    let output = match Command::new("rustc").arg("--version").output() {
        Ok(output) => output,
        Err(e) => {
            return println!("failed to execute process: {}", e);
        }
    };

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }
}

// pipes
static PANGRAM: &str = "the quick brown fox jumped over the lazy dog\n";

fn test_pipes() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(e) => return println!("couldn't spawn wc: {}", e),
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Ok(_) => println!("send pangram to wc"),
        Err(e) => return println!("couldn't write to wc stdin: {}", e),
    };

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Ok(_) => println!("wc responded with:\n{}", s),
        Err(e) => println!("couldn't read wc stdout: {}", e),
    }
}

// wait
fn test_wait() {
    let mut process = Command::new("sleep").arg("5").spawn().unwrap();

    let result = process.wait().unwrap();
    println!("reached end of test_wait");
}

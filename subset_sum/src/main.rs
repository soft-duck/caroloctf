use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut proc = Command::new("nc")
        .args(["caroloctf.ias.tu-bs.de", "13374"])
    // let mut proc = Command::new("./subset_sum_redacted")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn().unwrap();

    let stdin = proc.stdin.as_mut().unwrap();

    for i in -7..0 {
        stdin.write_all(b"2\n").unwrap();
        stdin.write_all(b"127\n").unwrap();
        stdin.write_all(format!("{}\n", 128 + i).as_bytes()).unwrap();
    }

    stdin.write_all(b"3\n").unwrap();

    let output = String::from_utf8(
        proc.wait_with_output().unwrap().stdout
    ).unwrap();

    let sums = output.split(&[' ', '\n'])
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mut flag = String::new();

    for i in 1.. sums.len() {
        let long = sums[i].overflowing_sub(sums[i - 1]).0;
        let bytes = long.to_le_bytes();

        flag.extend(bytes.iter().map(|v| *v as char));
    }

    println!("{}", flag);
}

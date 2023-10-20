use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
	let mut proc = Command::new("./subset_sum_redacted").stdout(Stdio::piped()).stdin(Stdio::piped()).spawn().unwrap();
	//let mut proc = Command::new("nc").args(["caroloctf.ias.tu-bs.de", "13374"]).stdout(Stdio::piped()).stdin(Stdio::piped()).spawn().unwrap();
	let mut stdin = proc.stdin.as_mut().unwrap();
	let out = proc.stdout.as_mut().unwrap();

	for i in 115..127 {
		stdin.write_all(&nth(i)).unwrap();
	}
	stdin.write_all(b"3\n").unwrap();
	let out = String::from_utf8(proc.wait_with_output().unwrap().stdout).unwrap();
	let nums = out.lines()
		.filter(|line| line.contains("sum ="))
		.map(|line| line.split("sum =").nth(1).unwrap().trim())
		//.enumerate().map(|(i, num)| format!("i: {i} {num}"))
		.map(|e|i64::from_str(e).unwrap() as u64)
		.map(|num|(0..8).into_iter().map(move |i|(num >> (i * 8)) as u8 as char))
		.flatten()
		.collect::<Vec<char>>();
	println!("{:?}", nums);
}

fn nth(i: usize) -> Vec<u8> {
	format!("2\n127\n{i}\n").into_bytes()
}
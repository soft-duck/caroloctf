use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
	let mut proc = Command::new("./subset_sum_redacted").stdout(Stdio::piped()).stdin(Stdio::piped()).spawn().unwrap();
	//let mut proc = Command::new("nc").args(["caroloctf.ias.tu-bs.de", "13374"]).stdout(Stdio::piped()).stdin(Stdio::piped()).spawn().unwrap();
	let mut stdin = proc.stdin.as_mut().unwrap();
	let out = proc.stdout.as_mut().unwrap();

	for i in 0..127 {
		stdin.write_all(&nth(i)).unwrap();
	}
	stdin.write_all(b"3\n").unwrap();
	let out = String::from_utf8(proc.wait_with_output().unwrap().stdout).unwrap();

	let mut counterweights = vec![0;127];
	let nums = out.lines()
		.filter(|line| line.contains("sum ="))
		.map(|line| line.split("sum =").nth(1).unwrap().trim())
		.map(|e|i64::from_str(e).unwrap())
		.collect::<Vec<_>>();
	println!("{:#?}", nums);
	let parsed = nums.iter()
		//.map(|num|(0..8).into_iter().map(move |i|(num >> (i * 8)) as u8 as char))
		//.flatten()
		//.enumerate().map(|(i, num)| format!("i: {} {num}", i))
		.enumerate()
		.map(|(i, e)|{
			let mut sum_before: i64 = 0;
			for e in &counterweights[0..i] {
				sum_before = sum_before.wrapping_add(*e);
			}
			counterweights[i] = e.wrapping_add(sum_before);
			e
		})
		.collect::<Vec<_>>();
	//dbg!(&counterweights);
	//dbg!(nums.into_iter().zip(counterweights.into_iter()).map(|(num, counterweight)|num.wrapping_sub(counterweight)).collect::<Vec<_>>());
}

fn nth(i: usize) -> Vec<u8> {
	format!("2\n127\n{i}\n").into_bytes()
}
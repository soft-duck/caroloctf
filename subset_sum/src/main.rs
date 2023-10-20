use std::io::{BufRead, Error, ErrorKind, Read, Stdin, stdin, stdout, Write};
use std::io::ErrorKind::WouldBlock;
use std::net::TcpStream;
use std::path::Path;
use std::process::{Command, exit};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, Thread};
use std::time::Duration;

fn main() {
	let stream = TcpStream::connect("caroloctf.ias.tu-bs.de:13374").unwrap();
	stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
	let nc = Arc::new(Mutex::new(stream));
	dbg!(&nc);
	// nc.lock().unwrap().write("1\n".as_bytes()).unwrap();

	let other = nc.clone();
	let receiver = thread::spawn(move || {
		const size: usize = 1;
		loop {
			let mut line = [0; size];
			let result = other.lock().unwrap().read_exact(&mut line);
			match result {
				Ok(_) => print!("{}", String::from_utf8(line.to_vec()).unwrap()),
				Err(e) => {
					match e.kind() {
						WouldBlock => {
							sleep(Duration::from_millis(1000));
						}
						_ => {
						}
					}
				}
			}
			line = [0; size];
			stdout().lock().flush().unwrap();
		}
	});

	loop {
		let mut str = String::new();
		let _ = stdin().lock().read_line(&mut str).unwrap();
		nc.lock().unwrap().write_all(format!("{str}\n").as_bytes()).unwrap();
		str.clear();
	}
}

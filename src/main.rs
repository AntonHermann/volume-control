use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::error::Error;

pub type Result<T> = std::result::Result<T, String>;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    println!("Welcome to system_monitor.rs");
    let name = "Master";

    let mut monitor = Command::new("sh")
        .args(&["-c", "stdbuf -oL alsactl monitor"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start alsactl monitor")
        .stdout
        .expect("Failed to pipe alsactl monitor output");

    let mut buffer = [0; 1024];
    loop {
        if let Ok(_) = monitor.read(&mut buffer) {
            print_sound_info(name)?;
        }
        thread::sleep(Duration::new(0,250_000_000))
    }
}

fn print_sound_info(name: &str) -> Result<()> {
    let output: String = Command::new("sh")
        .args(&["-c", format!("amixer get {}", name).as_str()])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .map_err(|e| e.description().to_owned())?;

    eprintln!("{}", output);
    let last = output.lines().last().ok_or("coulnd't get left channel")?;

    const FILTER_PATTERN: &[char] = &['[', ']', '%'];

    let mut els = last.split_whitespace().filter(|x| x.starts_with('['))
        .map(|s| s.trim_matches(FILTER_PATTERN));

    let vol = els.next().ok_or("coulnd't read volume")?.parse::<u32>()
        .map_err(|_| "failed parsing volume")?;

    let muted = els.next().ok_or("couldn't get muted state")
        .map(|s| s == "off")?;

    println!("Volume: {}, Muted: {}", vol, muted);

    Ok(())
}

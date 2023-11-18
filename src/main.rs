use std::env;
use std::process::Command;

fn kill_process_on_port(port: u16) {
    let output = match Command::new("lsof")
        .arg("-i")
        .arg(format!("tcp:{}", port))
        .arg("-t")
        .output() {
            Ok(output) if output.status.success() => output,
            _ => {
                eprintln!("No process is running! Please run killports again");
                return;
            }
        };

    let pids = String::from_utf8_lossy(&output.stdout);

    for pid in pids.split_whitespace() {
        if let Err(e) = Command::new("kill").arg(pid).output() {
            eprintln!("Failed to kill process with PID {}: {}", pid, e);
        } else {
            println!("Killed process with PID {} on port {}", pid, port);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: killports <port>");
        std::process::exit(1);
    }

    let port = args[1].parse::<u16>().expect("Invalid port number");
    kill_process_on_port(port);
}

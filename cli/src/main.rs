// cli/src/main.rs
use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::io::Read;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "esprog", version = "0.1", about = "ESP32-S3 programmer CLI")]
struct Cli {
    #[arg(short, long)]
    port: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Ping,
    Info,
    Erase,

    Read { address: String, length: String },

    Write { address: String, data: Vec<String> },

    Fill { value: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let port_name = match cli.port {
        Some(port) => port,
        None => find_port()?,
    };

    println!("Using port: {}", port_name);

    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_secs(1))
        .open()?;

    match cli.command {
        Command::Ping => {
            send(&mut *port, "PING")?;
        }

        Command::Info => {
            send(&mut *port, "INFO")?;
        }

        Command::Erase => {
            send(&mut *port, "ERASE")?;
        }

        Command::Read { address, length } => {
            send(&mut *port, &format!("READ {} {}", address, length))?;
        }

        Command::Write { address, data } => {
            send(&mut *port, &format!("WRITE {} {}", address, data.join(" ")))?;
        }

        Command::Fill { value } => {
            send(&mut *port, &format!("FILL {}", value))?;
        }
    }

    Ok(())
}

fn find_port() -> Result<String> {
    let ports = serialport::available_ports()?;

    for port in ports {
        let name = port.port_name;

        // Linux ESP32 USB CDC
        if name.starts_with("/dev/ttyACM") {
            return Ok(name);
        }

        // macOS ESP32 USB CDC
        if name.starts_with("/dev/cu.usbmodem") {
            return Ok(name);
        }

        // macOS older USB serial adapters
        if name.starts_with("/dev/cu.SLAB") {
            return Ok(name);
        }
    }

    Err(anyhow!(
        "No ESP32 USB serial device found. Try --port manually."
    ))
}

fn send(port: &mut dyn serialport::SerialPort, command: &str) -> Result<()> {
    use std::io::Write;

    port.write_all(command.as_bytes())?;

    port.write_all(b"\n")?;

    let mut buffer = [0u8; 512];

    match port.read(&mut buffer) {
        Ok(size) => {
            print!("{}", String::from_utf8_lossy(&buffer[..size]));
        }

        Err(e) => {
            eprintln!("Read error: {}", e);
        }
    }

    Ok(())
}

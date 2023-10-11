use std::env;
use std::net::TcpStream;

const COMMON_PORTS: [u16; 10] = [20, 21, 22, 23, 25, 80, 443, 587, 1433, 8080];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <program> <ip_range>");
        println!("Example: {} 192.168.1.x", args[0]);
        return;
    }

    let base_ip = &args[1][..args[1].len() - 1]; // Remove the 'x' from the end
    println!("Scanning on IP range: {}nnn:1-255", base_ip);

    for i in 1..255 {
        let ip = format!("{}{}", base_ip, i);
        scan_ip(&ip);
    }
}

fn scan_ip(ip: &str) {
    println!("\nScanning IP: {}", ip);

    let mut open_ports = Vec::new();

    for &port in &COMMON_PORTS {
        let ipaddr = format!("{}:{}", ip, port);
        let socket  = ipaddr.parse().unwrap();
        if TcpStream::connect_timeout(&socket, std::time::Duration::new(1, 0)).is_ok() {
            open_ports.push(port);
        }
    }

    if !open_ports.is_empty() {
        print!("Open ports on {}: ", ip);
        for port in &open_ports {
            print!("{} ", port);
        }
        println!();
    } else {
        println!("No open ports found on {}.", ip);
    }
}


use std::env;
use std::net::{SocketAddr, IpAddr};
use tokio::net::TcpStream;
use tokio::time::Duration;

const COMMON_PORTS: [u16; 10] = [20, 21, 22, 23, 25, 80, 443, 587, 1433, 8080];

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <program> <ip_range>");
        println!("Example: {} 192.168.1.x", args[0]);
        return;
    }

    let base_ip = &args[1][..args[1].len() - 1];
    println!("Scanning on IP range: {}[1-255]", base_ip);

    let mut handles = vec![];

    for i in 1..255 {
        let ip = format!("{}{}", base_ip, i).parse().unwrap();
        // Spawn a new asynchronous task for each IP address
        let handle = tokio::spawn(scan_ip(ip));
        handles.push(handle);
    }

    // Await all tasks to complete.
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn scan_ip(ip: IpAddr) {

    let mut open_ports = Vec::new();

    for &port in &COMMON_PORTS {
        let addr = SocketAddr::new(ip, port);
        // Use tokio::time::timeout to add a timeout to the TcpStream::connect operation.
        let result = tokio::time::timeout(
            Duration::from_secs(1),
            TcpStream::connect(&addr),
        ).await;

        if let Ok(Ok(_stream)) = result {
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
        //println!("No open ports found on {}.", ip);
    }
}



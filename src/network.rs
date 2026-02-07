// src/network.rs - FalconCore Native Network Stack (Advanced)
// Real TCP port scan + basic ARP lookup (placeholder)

use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::io;

pub struct NetworkStack;

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack
    }

    pub fn scan(&self, subnet: &str, port: u16) -> Vec<String> {
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();

            match TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
                Ok(_) => {
                    devices.push(ip.clone());
                    println!("Open: {}:{} (TCP)", ip, port);
                }
                Err(_) => {
                    // Closed or timeout - no action
                }
            }
        }
        devices
    }
// src/network.rs - FalconCore Network Stack (Advanced: real ARP parsing + multi-port scan)
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

pub struct NetworkStack;

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack
    }

    pub fn scan(&self, subnet: &str, ports: &[u16]) -> Vec<(String, Vec<u16>)> {
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            let mut open_ports = vec![];

            for &port in ports {
                let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
                if let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
                    open_ports.push(port);
                    drop(stream);
                }
            }

            if !open_ports.is_empty() {
                devices.push((ip, open_ports));
            }
        }
        devices
    }

    pub fn arp_table(&self) -> HashMap<String, String> {
        let mut table = HashMap::new();
        if let Ok(file) = File::open("/proc/net/arp") {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let parts: Vec<&str> = l.split_whitespace().collect();
                    if parts.len() > 3 && parts[0].contains('.') && parts[3].contains(':') && parts[3] != "00:00:00:00:00:00" {
                        table.insert(parts[0].to_string(), parts[3].to_string());
                    }
                }
            }
        }
        table
    }

    pub fn get_mac(&self, ip: &str) -> String {
        let table = self.arp_table();
        table.get(ip).cloned().unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn report(&self, devices: &[(String, Vec<u16>)]) {
        println!("\nNetwork Scan Report:");
        println!("Total devices with open ports: {}", devices.len());
        for (ip, ports) in devices {
            println!("IP: {} | Open Ports: {:?}", ip, ports);
            println!("   MAC: {}", self.get_mac(ip));
        }
    }
}
    pub fn arp_lookup(&self, ip: &str) -> String {
        // Real ARP needs root or /proc/net/arp
        // Placeholder for now
        format!("00:14:22:xx:xx:xx (ARP lookup placeholder for {})", ip)
    }
}// src/network.rs - FalconCore Native Network Stack (Advanced Step 1)
// Uses std::net for real TCP connect check + ARP placeholder

use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::io;

pub struct NetworkStack;

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack
    }

    pub fn scan(subnet: &str) -> Vec<String> {
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            let addr: SocketAddr = format!("{}:80", ip).parse().unwrap();

            if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_millis(50)) {
                devices.push(ip);
            } else {
                // Optional: ARP lookup (requires root or cache)
                // For now, we skip real ARP
            }
        }
        devices
    }

    pub fn get_mac(&self, ip: &str) -> String {
        // Placeholder for ARP lookup
        // Real implementation needs root or /proc/net/arp reading
        format!("00:14:22:xx:xx:xx (ARP placeholder for {})", ip)
    }
}// src/network.rs - FalconCore Native Network Stack (Step 1)
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct NetworkStack;

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack
    }

    pub fn scan(subnet: &str) -> Vec<String> {
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            if let Ok(mut stream) = TcpStream::connect_timeout(&ip, 80, Duration::from_millis(50)) {
                devices.push(ip);
            }
        }
        devices
    }
}// src/network.rs - FalconCore Native Network Stack (First Step)
// This will be the foundation for network.scan etc.

pub struct NetworkStack {
    // Future: sockets, TCP/IP, ARP cache etc.
}

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack {}
    }

    pub fn scan(&self, subnet: &str) -> Vec<String> {
        // Placeholder - real scan later
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            // Simulate scan
            if i % 10 == 0 {
                devices.push(ip);
            }
        }
        devices
    }
}
// src/network.rs - FalconCore Network Stack (Real ARP + Port Range Scan)
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::fs::File;
use std::io::{self, BufRead};

pub struct NetworkStack;

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack
    }

    pub fn scan(&self, subnet: &str, start_port: u16, end_port: u16) -> Vec<(String, Vec<u16>)> {
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            let mut open_ports = vec![];

            for port in start_port..=end_port {
                let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
                if let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
                    open_ports.push(port);
                    drop(stream);
                }
            }

            if !open_ports.is_empty() {
                devices.push((ip, open_ports));
            }
        }
        devices
    }

    pub fn arp_lookup(&self, ip: &str) -> String {
        // Read /proc/net/arp (Linux only, requires read permission)
        if let Ok(file) = File::open("/proc/net/arp") {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line {
                    if l.contains(ip) {
                        let parts: Vec<&str> = l.split_whitespace().collect();
                        if parts.len() > 3 {
                            return parts[3].to_string();
                        }
                    }
                }
            }
        }
        format!("Unknown (ARP lookup failed for {})", ip)
    }
}
// src/network.rs - FalconCore Network Stack (Real ARP parsing + port range scan)
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

pub struct NetworkStack;

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack
    }

    pub fn scan(&self, subnet: &str, start_port: u16, end_port: u16) -> Vec<(String, Vec<u16>)> {
        let mut devices = vec![];
        for i in 1..=254 {
            let ip = format!("{}.{}", subnet, i);
            let mut open_ports = vec![];

            for port in start_port..=end_port {
                let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
                if let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
                    open_ports.push(port);
                    drop(stream);
                }
            }

            if !open_ports.is_empty() {
                devices.push((ip, open_ports));
            }
        }
        devices
    }

    pub fn arp_table(&self) -> HashMap<String, String> {
        let mut table = HashMap::new();
        if let Ok(file) = File::open("/proc/net/arp") {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let parts: Vec<&str> = l.split_whitespace().collect();
                    if parts.len() > 3 && parts[0].contains('.') && parts[3].contains(':') {
                        table.insert(parts[0].to_string(), parts[3].to_string());
                    }
                }
            }
        }
        table
    }

    pub fn get_mac(&self, ip: &str) -> String {
        let table = self.arp_table();
        table.get(ip).cloned().unwrap_or_else(|| "Unknown".to_string())
    }
            }

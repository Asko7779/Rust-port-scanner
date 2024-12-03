use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};
use std::thread;
     
    // a basic rust port scanner i made, nothing too complex but works

fn port_scanning(tx: Sender<u16>, ip: &str, port: u16) {
    let addr = format!("{}:{}", ip, port);
    if TcpStream::connect(&addr).is_ok() {
        tx.send(port).unwrap();  }
}
fn main() {
    print!("[-] Input Target Host: ");
    io::stdout().flush().unwrap();
    let mut target_ip = String::new();
    io::stdin().read_line(&mut target_ip).unwrap();
    let target_ip = target_ip.trim();
    let port_range = 1..=65535;
    let thread_limit = 100;
    let (tx, rx) = channel();
    let mut handles = Vec::new();
    for port in port_range {
        let tx_clone = tx.clone();
        let ip = target_ip.to_string();
        let handle = thread::spawn(move || {
            port_scanning(tx_clone, &ip, port);
        } );
        
        handles.push(handle);
        if handles.len() >= thread_limit {
            for handle in handles.drain(..) {
                handle.join().unwrap();
            }
        } }
    for handle in handles {
        handle.join().unwrap(); }
    drop(tx);
    println!("[Open ports]");
    for port in rx {
        println!("[{}]", port);
    }
}
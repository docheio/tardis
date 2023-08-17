use std::net::SocketAddr;
use std::process::Command;
use std::{env, process};

use tokio::net::UdpSocket;

use tun_tap::{Iface, Mode};

fn cmd(cmd: &str, args: &[&str]) {
    let ecode = Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(ecode.success(), "Failed to execte {}", cmd);
}

async fn accept(iface: &Iface, socket: &UdpSocket) -> SocketAddr {
    let mut buffer = vec![0; 1504];
    let (len, addr) = socket.recv_from(&mut buffer).await.unwrap();
    iface.send(&mut buffer[4..len]).unwrap();
    return addr;
}

async fn loop_send(rem_address: &str, iface: &Iface, socket: &UdpSocket) {
    loop {
        let mut buffer = vec![0; 1504];
        let len = iface.recv(&mut buffer).unwrap();
        socket.send_to(&buffer[..len], &rem_address).await.unwrap();
    }
}

async fn loop_recv(iface: &Iface, socket: &UdpSocket) {
    loop {
        let mut buffer = vec![0; 1504];
        let (len, _) = socket.recv_from(&mut buffer).await.unwrap();
        iface.send(&mut buffer[4..len]).unwrap();
    }
}

pub async fn server() {
    // Read Local & Remote IP from args
    let loc_address = env::args().nth(2).expect("Unable to recognize listen IP");

    // Create interface
    let name = &env::args()
        .nth(3)
        .expect("Unable to configure the interface name");
    let tap = Iface::new(&name, Mode::Tap).unwrap_or_else(|err| {
        eprintln!("Failed to configure the interface: {}", err);
        process::exit(1);
    });

    // Configure the „local“ (kernel) endpoint.
    let ip = &env::args()
        .nth(4)
        .expect("Unable to recognize remote interface IP");
    cmd("ip", &["addr", "add", "dev", tap.name(), &ip]);
    cmd("ip", &["link", "set", "up", "dev", tap.name()]);

    // Create socket
    let socket = UdpSocket::bind(&loc_address).await.unwrap_or_else(|err| {
        eprintln!("Failed to open socket: {}", err);
        process::exit(1);
    });
    println!("ok0");
    // Handshake
    loop {
        let addr = accept(&tap, &socket).await;
        let _ = loop_send(&addr.to_string(), &tap, &socket);
        println!("ok");
        let _ = loop_recv(&tap, &socket);
    }
}

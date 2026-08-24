#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rustix::io::{read, write};
use rustix::net::sockopt::set_socket_reuseaddr;
use rustix::net::{
    AddressFamily, Ipv6Addr, SocketAddrV6, SocketType, accept, bind, listen, socket,
};
use rustix::runtime::exit_group;

const RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let listener = match socket(AddressFamily::INET6, SocketType::STREAM, None) {
        Ok(listener) => listener,
        Err(_) => exit_group(1),
    };

    // Activer la réutilisation d'adresse immédiatement après la création du socket
    if set_socket_reuseaddr(&listener, true).is_err() {
        exit_group(1);
    }

    let address = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 8080, 0, 0);
    if bind(&listener, &address).is_err() {
        exit_group(1);
    }

    if listen(&listener, 128).is_err() {
        exit_group(1);
    }

    loop {
        let connection = match accept(&listener) {
            Ok(connection) => connection,
            Err(_) => continue,
        };

        let mut request = [0u8; 1024];
        let _ = read(&connection, &mut request);

        let mut written = 0;
        while written < RESPONSE.len() {
            match write(&connection, &RESPONSE[written..]) {
                Ok(0) | Err(_) => break,
                Ok(count) => written += count,
            }
        }
    }
}

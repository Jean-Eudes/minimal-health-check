#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rustix::fd::AsFd;
use rustix::io::{read, write};
use rustix::net::sockopt::set_socket_reuseaddr;
use rustix::net::{
    AddressFamily, Ipv6Addr, SocketAddrV6, SocketType, accept, bind, listen, socket,
};
use rustix::runtime::exit_group;

const RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\nOK";

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}

fn setup_listener() -> Result<rustix::fd::OwnedFd, rustix::io::Errno> {
    let listener = socket(AddressFamily::INET6, SocketType::STREAM, None)?;
    set_socket_reuseaddr(&listener, true)?;

    let address = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 8080, 0, 0);
    bind(&listener, &address)?;
    listen(&listener, 128)?;

    Ok(listener)
}

fn serve_connection(connection: rustix::fd::BorrowedFd<'_>) {
    let mut request = [0u8; 1024];
    let _ = read(connection, &mut request);
    let _ = write(connection, RESPONSE);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let listener = setup_listener().unwrap_or_else(|_| exit_group(1));

    loop {
        if let Ok(connection) = accept(&listener) {
            serve_connection(connection.as_fd());
        }
    }
}

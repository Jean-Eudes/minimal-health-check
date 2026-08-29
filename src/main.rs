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

use core::mem::MaybeUninit; // Permet de déclarer de la mémoire non initialisée

fn serve_connection(connection: rustix::fd::BorrowedFd<'_>) {
    // 1. On dit à Rust de NE PAS remplir la mémoire de zéros
    let mut request = unsafe { MaybeUninit::<[u8; 1024]>::uninit().assume_init() };
    let _ = read(connection, &mut request);
    let _ = write(connection, RESPONSE);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 2. Utilisation d'un match direct pour éviter la sur-optimisation de unwrap_or_else
    let listener = match setup_listener() {
        Ok(l) => l,
        Err(_) => exit_group(1),
    };

    loop {
        if let Ok(connection) = accept(&listener) {
            serve_connection(connection.as_fd());
        }
    }
}

#![no_main]
#![no_std]

use core::mem::MaybeUninit;

use rustix::io::{read, write};
use rustix::net::sockopt::set_socket_reuseaddr;
use rustix::net::{
    AddressFamily, Ipv4Addr, SocketAddrV4, SocketType, accept, bind, listen, socket,
};

const RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\nOK";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 1. IPv4 strict (AF_INET) + Pas de gestion d'erreur (on force l'extraction via un unsafe/unwrap sauvage)
    let listener =
        unsafe { socket(AddressFamily::INET, SocketType::STREAM, None).unwrap_unchecked() };

    // Réutilisation du port
    let _ = set_socket_reuseaddr(&listener, true);

    // Écoute sur 0.0.0.0:8080 (Structure IPv4 minuscule)
    let address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8080);
    let _ = bind(&listener, &address);
    let _ = listen(&listener, 128);

    let request = MaybeUninit::<[u8; 120]>::uninit();
    let mut request = unsafe { request.assume_init() };
    // Boucle infinie brute
    loop {
        // On accept sans vérifier si la connexion est valide
        let connection = unsafe { accept(&listener).unwrap_unchecked() };
        let _ = read(&connection, &mut request);
        let _ = write(&connection, RESPONSE);
    }
}

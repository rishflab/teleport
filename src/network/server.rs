use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Server {
    socket: UdpSocket,
    buffer_size: u32,
}

impl Server {
   pub fn new(port: u16) -> Server {

    //    let addr = SocketAddr::from()

        let mut socket = UdpSocket::bind("127.0.0.1:34254").expect("could not bind socket");

        
        // let mut socket = match UdpSocket::bind("127.0.0.1:34254") {
        //     Ok(socket) => socket,
        //     Err(e) => panic!("couldn't bind socket"),
        // };

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src).expect("could not send to socket");

        Server {
            socket: socket,
            buffer_size: 1024,
        }
   }


//    fn handle_client(mut stream: TcpStream) {
//        let buf = String::new();
//        let incoming = stream.read_to_string(&mut buf.clone()).unwrap();
//    //    match incoming {
//    //        Ok(t) => {
//    //            println!("{}", t);
//    //        },
//    //        Err(e) => {
//    //            println!("{}", e);
//    //        }
//    //    }
//        println!("{:?}", incoming)
//    }


}
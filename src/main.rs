use hex;
use std::net::{UdpSocket};

fn main() -> std::io::Result<()> {
    {
        // Prepare socket
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        socket
            .set_broadcast(true)
            .expect("Cannot set broadcast mode");


        // Build a magic sequence
        let preamble = "FFFFFFFFFFFF";
        let raw_mac = std::env::args().nth(1).expect("usage: liftmeup aa:bb:cc:dd:ee:ff");

        let mac = raw_mac.replace(":","");
        assert_eq!(12, mac.len(), "MAC length should be 12 symbols (: are ignored)");

        let repeated_mac = mac.repeat(16);

        let magic_seq = hex::decode(format!("{}{}", preamble, repeated_mac)).expect("couldn't parse mac address");

        socket.send_to(&magic_seq[..], "255.255.255.255:9").expect("couldn't send message");;
    
    } 
    Ok(())
}

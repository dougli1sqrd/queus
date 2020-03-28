
extern crate queus;

use queus::Packet;
use queus::device::DevicePath;
// use queus::system::NetworkNode;
// use queus::system::Networked;


fn main() {
    println!("Hello, world!");

    let m = Packet::End;
    println!("{:?}", m);

    let device_path: DevicePath = "/main/net1/term".into();
    
    
}

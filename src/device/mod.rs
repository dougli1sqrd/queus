use std::sync;
use super::Packet;
use std::fmt;

struct Process {
    input: sync::mpsc::Receiver<Packet>,
    output: sync::mpsc::Sender<Packet>
}

trait Run {
    fn run();
}

pub struct Device {
    id: String,
    connected_devices: Vec<Device>,
    parent_device: Option<Box<Device>>
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DevicePath {
    pub path: Vec<String>
}

impl From<&str> for DevicePath {
    fn from(s: &str) -> DevicePath {
        let path = s.split("/").map(|part| String::from(part) ).filter(|p| !p.is_empty() ).collect();
        DevicePath {
            path: path
        }
    }
}

impl fmt::Display for DevicePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.join("/"))
    }
}

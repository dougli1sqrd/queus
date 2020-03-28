use super::device::DevicePath;
use super::Packet;

pub struct DeviceMessage {
    to: DevicePath,
    from: DevicePath,
    contents: Vec<Packet>
}
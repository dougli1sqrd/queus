use std::collections::HashMap;
use std::hash::Hash;

use crate::device::DevicePath;


struct System {
    mainframe: Address,
    terminal: Address,
    network: Network<Device>,
}

impl System {
    fn new() -> System {
        let mut network: Network<Device> = Network::new();
        let mainframe = Mainframe {
            id: "main".into(),
            address: Address(1)
        };
        let main_address = mainframe.address;
        network.connect_with_address(Device::Mainframe(mainframe), main_address, None);

        let term_main_netnode = NetworkNode {
            id: "net3".into(),
            address: Address(3)
        };
        let netaddress = network.connect_with_root(Device::NetworkNode(term_main_netnode), None);

        let terminal = Terminal {
            id: "term1".into(),
            address: Address(2)
        };
        let terminal_address = terminal.address;
        network.connect_with_address(Device::Terminal(terminal), terminal_address, Some(netaddress));

        System {
            mainframe: main_address,
            terminal: terminal_address,
            network: network
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Address(u16);

struct Counter {
    value: u16
}

impl Counter {
    fn new() -> Counter {
        Counter {
            value: 1000
        }
    }
}

impl Iterator for Counter {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        Some(self.value)
    }
}

pub struct Network<N> {

    counter: Counter,

    /// Root of the Tree
    root: Option<Address>,

    /// The list of Nodes in the network, owned by this list
    nodes: HashMap<Address, N>,

    /// For any node N, this gets the list of nodes Vec<N> that the node
    /// is connected to as children.
    connections: HashMap<Address, Vec<Address>>,

    /// For any node N, this gets any node that is its parent (if it has one)
    parent_connection: HashMap<Address, Address>
    
}

impl<N: Eq + Hash> Network<N> {

    fn new() -> Network<N> {
        Network {
            counter: Counter::new(),
            root: None,
            nodes: HashMap::new(),
            connections: HashMap::new(),
            parent_connection: HashMap::new()
        }
    }

    ///
    /// This will add a `new_node`, `N` into the network. A parent address may be supplied,
    /// and if it is the `new_node` will be a child of the `parent`. Additionally the 
    /// parent is added as the `parent` in the `parent_connection` map.
    /// 
    /// The address that was assigned to the node is returned.
    /// 
    fn connect_to_parent(&mut self, new_node: N, parent: Option<Address>) -> Address {
        // First transfer ownership to `self.nodes` and then grab a reference to it.
        // If we supplied a parent, then add the `new_node` to the parent's children
        // If parent wasn't in the map, put it there
        // Add the parent as the `new_node` parent connection

        let new_address = Address(self.counter.next().unwrap());
        self.connect_with_address(new_node, new_address, parent)
    }

    fn connect_with_root(&mut self, new_node: N, parent: Option<Address>) -> Address {
        if self.nodes.is_empty() {
            self.connect_to_parent(new_node, None)
        } else {
            match parent {
                None => { self.connect_to_parent(new_node, self.root) },
                Some(p) => { self.connect_to_parent(new_node, Some(p)) }
            }
        }
    }

    fn connect_with_address(&mut self, new_node: N, new_address: Address, parent: Option<Address>) -> Address {
        self.nodes.insert(new_address, new_node);

        // Assign root if we haven't yet (i.e. this is the first node)
        match self.root {
            None => { self.root = Some(new_address); }
            _ => {}
        }

        if let Some(p) = parent {
            match self.connections.get_mut(&p) {
                Some(children) => { children.push(new_address) },
                None => { self.connections.insert(p, vec![new_address]); }
            }
            self.parent_connection.insert(new_address, p);
        }
        self.connections.insert(new_address, Vec::new());
        return new_address
    }

    fn get_node(&self, address: Address) -> Option<&N> {
        self.nodes.get(&address)
    }

    fn get_parent(&self, address: Address) -> Option<&Address> {
        self.parent_connection.get(&address)
    }

    fn get_children(&self, address: Address) -> Option<&Vec<Address>> {
        self.connections.get(&address)
    }
}

// trait Networked {
//     fn address(&self) -> Address;
// }

#[derive(Hash, PartialEq, Eq, Debug)]
struct NetworkNode {
    id: String,
    address: Address,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Mainframe {
    id: String,
    address: Address,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Terminal {
    id: String,
    address: Address
}

trait KnowsPath: Addressable {
    fn device_path(&self, network: &Network<Device>) -> DevicePath;
}

trait Addressable {
    fn address(&self) -> Address;
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Device {
    Mainframe(Mainframe),
    NetworkNode(NetworkNode),
    Terminal(Terminal)
}

impl Device {
    fn id(&self) -> String {
        match self {
            Device::Mainframe(main) => main.id.clone(),
            Device::NetworkNode(net) => net.id.clone(),
            Device::Terminal(term) => term.id.clone()
        }
    }
}

impl Addressable for Device {

    fn address(&self) -> Address {
        match self {
            Device::Mainframe(main) => main.address,
            Device::NetworkNode(net) => net.address,
            Device::Terminal(term) => term.address,
        }
    }
}

impl KnowsPath for Device {
    fn device_path(&self, network: &Network<Device>) -> DevicePath {
        let mut path: Vec<String> = Vec::new();
        
        let mut current = self;
        path.push(current.id());
        while let Some(p) = network.get_parent(current.address()) {
            println!("current: {:?}, id = {}", current, current.id());
            path.push(current.id());
            match network.get_node(*p) {
                Some(d) => {
                    current = d;
                },
                None => {
                    println!("woops");
                    break;
                }
            }
        }
        path.reverse();
        DevicePath {
            path: path
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Module {
    id: String,
    address: Address,
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct TestNode(String);

    #[test]
    fn test_new_network() {

        let network: Network<TestNode> = Network::new();
        assert_eq!(network.nodes.is_empty(), true);
        assert_eq!(network.root, None);
    }

    #[test]
    fn test_adding_root_node() {
        let mut network: Network<TestNode> = Network::new();
        let root_address = network.connect_to_parent(TestNode("Hello".into()), None);


        assert_eq!(network.root.unwrap(), root_address);
        assert_eq!(network.get_node(root_address), Some(&TestNode("Hello".into())));
        assert_eq!(network.get_children(root_address), Some(&Vec::new()));
        assert_eq!(network.get_parent(root_address), None);
    }

    #[test]
    fn test_child_and_parent() {
        let mut network: Network<TestNode> = Network::new();
        let root = network.connect_to_parent(TestNode("Hello".into()), None);
        // Add a child node
        let child = network.connect_to_parent(TestNode("World".into()), Some(root));

        assert_eq!(network.get_children(root), Some(&vec![child]));
        assert_eq!(network.get_parent(child), Some(&root));
    }

    #[test]
    fn test_connecting_with_root() {
        let mut network: Network<TestNode> = Network::new();
        let root = network.connect_to_parent(TestNode("Hello".into()), None);
        let child = network.connect_with_root(TestNode("World".into()), None);

        assert_eq!(network.get_children(root), Some(&vec![child]));
        assert_eq!(network.get_parent(child), Some(&root));
    }

    #[test]
    fn test_system_structure() {
        let system = System::new();

        assert_eq!(system.mainframe, Address(1));

        let grandparent = system.network.get_parent(system.terminal)
            .and_then(|p| system.network.get_parent(*p)); // Should be root (mainframe)
        
        assert_eq!(grandparent, Some(&system.mainframe));
        
        // assert_eq!(system.network.get_children(system.mainframe)[0])
    }

    #[test]
    fn test_device_path() {
        let system = System::new();
        println!("nodes {:?}", system.network.nodes);
        println!("parents {:?}", system.network.parent_connection);
        println!("children {:?}", system.network.connections);

        let mainframe = system.network.get_node(system.mainframe).unwrap();
        assert_eq!(mainframe.device_path(&system.network), "/main".into());
    }
}



// impl<'a> NetworkNode<'a> {

//     fn new() -> NetworkNode<'a> {
//         NetworkNode {
//             device: None,
//             parent: None,
//             children: Vec::new()
//         }
//     }

//     fn _parent(&mut self, parent_node: &'a NetworkNode) -> &Self {
//         self.parent = Some(parent_node);
//         self
//     }

//     fn _add_child(&mut self, child: &'a NetworkNode) -> &Self {
//         self.children.push(child);
//         self
//     }

//     fn with_device(&mut self, device: &'a dyn Networked) -> &Self {
//         self.device = Some(device);
//         self
//     }

//     fn connect(&'a mut self, other: &'a mut NetworkNode<'a>) -> &Self {
//         self._add_child(other);
//         other._parent(self);
//         self
//     }
// }

// fn connect<'a>(existing: &'a mut NetworkNode<'a>, other: &'a mut NetworkNode<'a>) {
//     other._parent(existing);
//     existing._add_child(other);
// }

// pub trait Networked {
//     fn network_node(&self) -> &NetworkNode;
// }

// pub struct Module<'a> {
//     network: &'a NetworkNode<'a>,
//     id: u32
// }

// impl<'a> Module<'a> {
//     fn new(id: u32, network: &'a NetworkNode) -> Module<'a> {
//         Module {
//             network: network,
//             id: id
//         }
//     }
// }

// impl<'a> Networked for Module<'a> {
//     fn network_node(&self) -> &NetworkNode {
//         self.network
//     }
// }

// struct Mainframe<'a> {
//     network: &'a NetworkNode<'a>
// }

// impl<'a> Networked for Mainframe<'a> {
    
//     fn network_node(&self) -> &NetworkNode {
//         self.network
//     }
// }


// pub struct Device<T> {
//     dev: T,
//     parent: Option<Box<T>>,
//     children: Vec<Device<T>>
// }

// impl<T> Device<T> {
    
// }


// pub struct HelloThings<'a> {
//     pub hellos: Vec<&'a dyn Hello>
// }

// trait Device {

//     fn parent(&self) -> Option<& dyn Device>;

//     fn children(&self) -> Vec<& dyn Device>;

//     fn _add_child(&mut self, device: &mut dyn Device);

//     fn _set_parent(&mut self, device: &mut dyn Device);

//     fn connect(&mut self, device: &mut dyn Device) -> &dyn Device {
//         self._add_child(device);
//         self
//     }
// }

// fn connect<D: Device, E: Device>(&mut dev1: D, &mut dev2: E) -> Option<D> {
//     dev1.connect(dev2);
// }
mod network {
    pub mod protocol {
        pub fn tcp() {
            println!("TCP");
        }

        pub fn udp() {
            println!("UDP");
        }
    }
}

pub use network::protocol;

pub mod client {
    use crate::module::network::protocol;

    pub fn connect() {
        protocol::tcp();
    }
}

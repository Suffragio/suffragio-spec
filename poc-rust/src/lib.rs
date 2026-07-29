pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/suffragio.v1.rs"));
}

pub mod app;
pub mod ballot;
pub mod crypto;
pub mod services;
pub mod tally;
pub mod transport;

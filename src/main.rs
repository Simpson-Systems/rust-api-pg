#[cfg(feature = "ingestor")]
pub mod ingestor;

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "worker")]
pub mod worker;

fn main() {
    println!("Hello, world!");
}

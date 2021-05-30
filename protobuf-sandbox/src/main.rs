use serde::{Deserialize, Serialize};
mod protos;
use protobuf::Message;

use protos::a::GetResponse;

#[derive(Debug, Serialize, Deserialize)]
struct Mascot {
    name: String,
    species: String,
    year_of_birth: u32,
}

fn main() {
    println!("Hello, world!");
    Mascot{}.
}

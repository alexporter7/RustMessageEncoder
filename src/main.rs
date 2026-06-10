use crate::message::message::Message;

pub mod message;
pub mod menu;

fn main() {
    
    let d = Message::from("test.bin");

    println!("=== FROM FILE ===\n{d}");
}

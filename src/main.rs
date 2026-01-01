use canbus_common::interface::{read_counter, write_counter};



fn main() {
    println!("Hello, world!");
    read_counter(3);
    write_counter(3);
    write_counter(3);
    read_counter(3);

}
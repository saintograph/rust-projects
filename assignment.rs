// use std::{i8,i16,i32,i64,u8,u16,u32,u64,f32,f64,isize,usize};
// use std::io::stdin;
use std::{i32};

fn main() {
    println!("Variable assignment");
    let num = 10;
    println!("Num is: {}", num);

    let age: i32 = 40;
    println!("Age is {}", age);
    println!("Max i32 {}", i32::MAX);
    println!("Max i32 {}", i32::MIN);

    let(first_name, last_name) = ("Winfred", "Selwyn");
    println!("First name {0} last name {1}", first_name, last_name);
}
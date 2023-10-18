extern crate stack;

use stack::stack::interface::IStack;
use stack::stack::service::Stack;

fn main() {
    let mut stack = Stack::<i32>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Is Empty? {}", stack.is_empty());
    println!("Top: {}", stack.top().unwrap());
    println!("Pop: {}", stack.pop().unwrap());
    println!("Pop: {}", stack.pop().unwrap());
    println!("Pop: {}", stack.pop().unwrap());
    if let Some(item) = stack.pop() {
        println!("Pop: {}", item);
    } else {
        println!("Stack is empty");
    }
}

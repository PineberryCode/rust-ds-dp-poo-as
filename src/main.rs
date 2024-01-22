mod poo;
mod data_structure;

//use std::io;
//use crate::poo::composition;
//use crate::poo::agregation;
use crate::data_structure::{stack, queue};

fn main() {
    let mut queue = queue::Queue {
        front: None,
        rear: None,
        length: 0
    };
    queue.enqueue(String::from("Hawaiana"));
    queue.enqueue(String::from("Peperoni"));
    queue.enqueue("Extra-Cheese".to_string());
    queue.peek();
    queue.show_front();
    queue.show_rear();
    queue.dequeue();
    println!("Then dequeue");
    queue.show_front();
    queue.show_rear();
    println!("Length: {}", queue.length);
    
    /*
     * Stack
     */
    println!("*-*-*-*-* Stack *-*-*-*-*");
    let mut stack = stack::Stack {
        top: None,
        length: 0
    };

    stack.push(3);
    stack.push(4);
    stack.peek();
    println!("Length of the Stack: {}", stack.length());
    stack.show_top();
    stack.pop();
    println!("Then...");
    stack.peek();
    println!("Length of the Stack: {}", stack.length());
    stack.show_top();
    

    /*
     * agregation
    */
    /*let mut id = String::new();
    println!("Enter the ID:");
    let _ = io::stdin().read_line(&mut id).expect("Error reading input.");
    let id = id.trim().chars().next().expect("Couldn't you enter any character.");
    
    let mut code_security = String::new();
    println!("Enter the code security:");
    let _ = io::stdin().read_line(&mut code_security).expect("Error reading input.");
    let code_security: u8 = code_security.trim().parse().expect("Couldn't you enter any character.");

    let targets = agregation::add_target(id, code_security);

    println!("Debug => {:?}", targets);*/
}
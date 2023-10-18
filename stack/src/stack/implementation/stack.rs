use std::fmt::Display;

use crate::stack::interface::IStack;

struct Node<T> 
where
    T: Display + Copy
{
    item: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> 
where 
    T: Display + Copy
{
    fn new(item: T, next: Option<Box<Node<T>>>) -> Self {
        Self {
            item,
            next
        }
    }
}

pub struct ConcreteStack<T> 
where
    T: Display + Copy
{
    head: Option<Box<Node<T>>>
}

impl<T> ConcreteStack<T>
where
    T: Display + Copy
{
    pub fn new() -> Self {
        Self {
            head: None
        }
    }
}

impl<T> IStack<T> for ConcreteStack<T>
where
    T: Display + Copy 
{

    fn push(&mut self, item:T) {
        let head = self.head.take();
        let node = Node::new(item, head);
        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(head) = &mut self.head {
            let item = head.item;
            let head = head.next.take();
            self.head = head;
            return Some(item);
        } else {
            return None;
        }  
    }

    fn top(&self) -> Option<T> {
        if let Some(head) = &self.head {
            return Some(head.item);
        } else {
            return None;
        }  
    }

    fn is_empty(&self) -> bool {
        if let Some(_head) = &self.head {
            return true;
        } else {
            return false;
        }
    }

}
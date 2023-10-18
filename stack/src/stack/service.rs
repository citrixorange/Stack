use std::fmt::Display;

use crate::stack::interface::IStack;
use crate::stack::implementation::stack::ConcreteStack;

pub struct Stack<T> 
where
    T: Display + Copy
{
    stack: Box<dyn IStack<T>>
}

impl<T> Stack<T> 
where
    T: Display + Copy + 'static
{
    pub fn new() -> Self {
        let stack = ConcreteStack::<T>::new();
        Self {
            stack: Box::new(stack)
        }
    }
}

impl<T> IStack<T> for Stack<T> 
where
    T: Display + Copy
{

    fn push(&mut self, item:T) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    fn top(&self) -> Option<T> {
        return self.stack.top();
    }

    fn is_empty(&self) -> bool {
        return self.stack.is_empty();
    }

}
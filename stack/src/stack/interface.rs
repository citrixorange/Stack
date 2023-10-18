pub trait IStack<T> {
    fn push(&mut self, item:T);
    fn pop(&mut self) -> Option<T>;
    fn top(&self) -> Option<T>;
    fn is_empty(&self) -> bool;
}
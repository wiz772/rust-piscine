use std::{alloc::{Layout, alloc, dealloc}, ptr};


pub struct LinkedList<T>{
    head: *mut Node<T>,
    len: usize,
}

pub struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

impl<T> Node<T>{

    fn new(value: T) -> Node<T> {
        Node { 
            value: value, 
            next: std::ptr::null_mut() 
        }
    }

}

impl<T> LinkedList<T>{
    fn new() -> LinkedList<T>{
        LinkedList{
            head: std::ptr::null_mut(),
            len: 0,
        }
    }

    fn is_empty(&self) -> bool{
        self.len == 0
    }

    fn push_front(&mut self, value: T) {

        let node_layout: Layout = Layout::new::<Node<T>>();
        let mut new_node = Node::new(value);

        unsafe {

            let node_ptr = alloc(node_layout) as *mut  Node<T>;
            if node_ptr.is_null() {
                panic!("echec.")
            }
            new_node.next = self.head;

            ptr::write(node_ptr, new_node);
            self.head = node_ptr;



        }

    }
}

fn main() {
    println!("Hello, world!");
}

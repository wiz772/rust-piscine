use std::{alloc::{Layout, alloc, dealloc}, fmt::{Debug, Display}, ptr};


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

            self.len += 1;


        }

    }

    fn pop_front(&mut self){

        unsafe {

            let add_to_drop = self.head.add(0);



        }

    }

    fn print_list(&self) where T: std::fmt::Debug {

        let mut current = self.head;

        unsafe{

            while !current.is_null() {
                let node = &*current;

                println!("node value: {:?}, next: {:p}", node.value, node.next);

                current = node.next;
            }
        }
    }
}

fn main() {

    let mut list = LinkedList::new();
    list.push_front(String::from("caca"));

    list.push_front(String::from("coucou"));

    list.print_list();

}

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

    fn len(&self) -> usize {

        self.len
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

        if self.head.is_null() {
            return;
        }

        unsafe {    
            let old_head = self.head;

            let new_ptr: *mut Node<T> = (*old_head).next;
            self.head = new_ptr;

            let node_layout: Layout = Layout::new::<Node<T>>();

            ptr::drop_in_place(old_head);
            dealloc(old_head as *mut u8, node_layout);
            
            self.len -= 1;

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

    fn front(&self) -> Option<&T> {
        if self.head.is_null() {
            return None;
        }
        
        unsafe {

            let value = &(*self.head).value;
            Some(value)

        }

    }

    fn get_last_node(&self) -> Option<*mut Node<T>>{

        if self.is_empty(){
            return None;
        }

        let mut current = self.head;

        unsafe{
            while !(*current).next.is_null() {
                let node = &*current;
                current = node.next;
            }

            Some(current)
        }
    }

    fn push_back(&mut self, value: T){
        let node_layout: Layout = Layout::new::<Node<T>>();
        let mut new_node: Node<T> = Node::new(value);

        unsafe{
            let node_ptr = alloc(node_layout) as *mut Node<T>;
        
            if node_ptr.is_null(){
                panic!("Echec");
            }

            ptr::write(node_ptr, new_node);

            if self.is_empty(){
                self.head = node_ptr;

            } else {
                let last_node = self.get_last_node();

                if let Some(node) = last_node{

                    (*node).next = node_ptr;
                }
            }

            self.len += 1;
        }
    }

    fn pop_back(&mut self) {
        if self.is_empty() {
            return;
        }

        if self.len == 1 {
            self.clear();
            return;
        }

        unsafe {
            let mut current = self.head;

            while !(*(*current).next).next.is_null() {
                current = (*current).next;
            }

            let last_node = (*current).next;

            (*current).next = ptr::null_mut();

            ptr::drop_in_place(last_node);
            let node_layout = Layout::new::<Node<T>>();
            dealloc(last_node as *mut u8, node_layout);

            self.len -= 1;
        }
    }

    fn back(&self) -> Option<&T> {

        if self.head.is_null() {
            return None;
        }

        unsafe {

            let last_node = self.get_last_node();

            if let Some(node) = last_node{

                let value = &(*node).value;

                return Some(value);

            }

            None
        
        }

    }

    fn clear(&mut self) {

        while !self.is_empty() {

            self.pop_front();

        }

    }

}

impl<T> Drop for LinkedList<T> {

    fn drop(&mut self){

        self.clear();

    }

}

fn main() {

    let mut list = LinkedList::new();

    list.push_front(String::from("caca"));
    
    list.push_front(String::from("coucou"));

    list.print_list();

    println!("Back: {:?}", list.back());

    list.pop_back();

    println!("Back: {:?}", list.back());


}
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;


pub struct DynamicArray<T>{
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> DynamicArray<T> {

    fn new() ->  DynamicArray<T> {
        DynamicArray {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize{
        self.capacity
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    fn push(&mut self, value: T){

        if self.is_full() {
            let to_drop_layout = Layout::array::<T>(self.capacity).unwrap();

            let mut new_capacity = match self.capacity.checked_mul(2).and_then(|value| value.checked_add(1)) {
                Some(value) => value,
                None => panic!("trop grand lv"),
            };
            
            let layout: Layout = Layout::array::<T>(new_capacity).unwrap();
            
            unsafe{

                let new_ptr = alloc(layout) as *mut T;
                if new_ptr.is_null() {
                    panic!("échec");
                }

                let new_index = self.len;
                let target = new_ptr.add(new_index);

                if self.len > 0 {
                    ptr::copy(self.ptr, new_ptr, self.len);
                }
            
                ptr::write(target, value);

                if self.capacity > 0 {
                    dealloc(self.ptr as *mut u8, to_drop_layout);
                }

                self.capacity = new_capacity;
                self.len +=1;
                
                self.ptr = new_ptr;

                println!("capacity: {}", self.capacity);

            }
        } else{
            unsafe {
                let target = self.ptr.add(self.len);
                ptr::write(target, value);
                self.len += 1;
            }
        }
    }

    fn pop(&mut self){

        if self.is_empty(){
            return;
        }

        println!("pop index: {}", self.len - 1);

        unsafe{
            let last_element = self.ptr.add(self.len - 1);
            ptr::drop_in_place(last_element);
        }

        let new_len: usize = self.len - 1;
        self.len = new_len;

    }

    fn get(&self, index: usize) -> Option<&T>{

        if index >= self.len {
                return None;
        }

        unsafe{
            let value = &*self.ptr.add(index);
            
            Some(value)
        }        
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T>{

        if index >= self.len {
                return None;
        }

        unsafe{
            let value = &mut *self.ptr.add(index);

            Some(value)
        }        
    }

    fn clear(&mut self){
        unsafe{
            for i in 0..self.len{
                ptr::drop_in_place(self.ptr.add(i));
            }
        }

        self.len = 0;
        
    }

    fn reserve(&mut self, value: usize){

        let remaining_mem = self.capacity - self.len;
        if remaining_mem >= value {
            return;
        }

        let need_to_alloc = value - remaining_mem;
        
        let new_capacity = match self.capacity.checked_add(need_to_alloc) {
            Some(value) => value,
            None => panic!("trop grand lv"),
        };

        let layout = Layout::array::<T>(new_capacity).unwrap();


        let to_drop_layout: Layout = Layout::array::<T>(self.capacity).unwrap();

        unsafe {

            let new_ptr = alloc(layout) as *mut T;
            if new_ptr.is_null() {
                panic!("échec");
            }

            if self.len > 0 {
                ptr::copy(self.ptr, new_ptr, self.len);
            }

            if self.capacity > 0 {
                dealloc(self.ptr as *mut u8, to_drop_layout);
            }

            self.ptr = new_ptr;

        }

        self.capacity = new_capacity;
    

    }
    

}

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self){

        println!("début drop: len={}, capacity={}", self.len, self.capacity);

        self.clear();
        unsafe{

            if self.capacity > 0 {

                let to_drop_layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, to_drop_layout);
                
                println!("mem libérée");
            }

        }
        println!("fin drop");
    }
}


fn main() {
    let mut array = DynamicArray::new();

    array.clear();
    array.push(String::from("bonjour"));
    array.clear();
    println!("len={}, capacity={}", array.len(), array.capacity());

    array.reserve(1000);

    for i in 0..1000{
        array.push(i.to_string());
    }

}

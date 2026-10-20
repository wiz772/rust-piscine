pub struct HashMap<K, V>{

    key_values: Vec<Option<(K, V)>>
    len: usize,
    capacity: usize,

}

impl<K, V>  HashMap<K, V>{

    fn new() -> HashMap<K, V> {
        HashMap{
            key_values: Vec::new(),
            len: 0,
            capacity: 0
        }


    }

}

fn main() {
    println!("Hello, world!");
}

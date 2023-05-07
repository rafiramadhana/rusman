pub trait Animal {
    fn walk(&self) {
        println!("Walking...")
    }
}

pub struct Dog {}

impl Animal for Dog {}

pub struct Cat {}

impl Animal for Cat {
    fn walk(&self) {
        println!("Cat is walking without any excitement...")
    }
}

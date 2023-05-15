struct List {
    // Error because compiler does not know the size of List
    // item: Option<List>,

    // OK
    item: Option<Box<List>>,
}

fn drop_me<T>(_: T) {}

fn main() {
    let num = 1;
    drop_me(num);
    drop_me(num);

    let bx = Box::new(1);
    drop_me(bx.clone());
    // drop_me(bx);
    println!("{:?}", bx);
    println!("{:?}", *bx); // Box can also be "dereferenced"

    // TODO: Add "Box around traits"
}

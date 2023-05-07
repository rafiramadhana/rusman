struct Books {
    books: Vec<String>,
}

impl Iterator for Books {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"), // Rust allows String + &str
            None => None,
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1.iter().map(|x| x + 1).collect::<Vec<i32>>();

    let mut v3 = vec![4, 5, 6];
    v3.iter_mut().for_each(|x| *x += 10);

    println!("v1={:?} v2={:?} v3={:?}", v1, v2, v3);

    // Under the hood, iterator is just a .next() call
    let v1 = vec![7, 8, 9];
    let mut v1_iter = v1.iter();
    loop {
        match v1_iter.next() {
            Some(v) => println!("{}", v),
            _ => {
                println!("EXIT LOOP");
                break;
            }
        }
    }

    // Implementing iterators
    let bks = Books {
        books: vec!["Unix".to_string(), "Linux".to_string()],
    };

    for b in bks {
        println!("{}", b);
    }
}

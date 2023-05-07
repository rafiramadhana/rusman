mod animal;

fn main() {
    let x = animal::Dog {};
    process_animal(x);

    let x = animal::Cat {};
    process_animal(x);
}

fn process_animal(anm: impl animal::Animal) {
    anm.walk()
}

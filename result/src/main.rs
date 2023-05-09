fn main() {
    // let name: &str = "Rusman";
    let name = "Julio";

    match get_hero(name) {
        Ok(h) => println!("Got hero {}!", h),
        Err(err) => println!("Error: {}", err),
    }
}

fn get_hero(name: &str) -> Result<String, String> {
    if name.to_lowercase() == "rusman" {
        return Ok("Legion Commander".to_string());
    }
    Err("hero not found".to_string())
}

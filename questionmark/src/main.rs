fn main() {
    println!("{:?}", do_question_mark());
}

fn do_result() -> Result<i32, String> {
    Ok(0)
    // Err("Failed to do do_result".to_string())
}

fn do_question_mark() -> Result<i32, String> {
    let res = do_result()?;
    Ok(res)
}

use std::io;

fn main() {
    println!("Input a mathematical expression.");

    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    
    println!(r#"{expr}"#);
}
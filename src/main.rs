use std::io;

fn main() {
    println!("Show");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Не то");
    
    println!("Excpet: {}", guess);
}

// напомнить мне

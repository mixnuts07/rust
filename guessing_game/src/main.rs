use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");

    // String::new()関数を呼び出し、String型の新しいインスタンスを作成する
    // ::は関連関数（static method）を呼び出している
    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {}", guess);
}

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

    // 所有権と関数
    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s); // sの値が関数にムーブされた
    let x = 5; // xがスコープに入る
    makes_copy(x); // xも関数にムーブされるが、i32はCopyなので、この後にxを使っても大丈夫
}

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing);
} // ここでsome_thingがスコープを抜け、dropが呼ばれる。メモリが開放される。
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
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

    let mut user1 = User {
        email: String::from("someone@gmail.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String.from("another@gmail.com"); // インスタンス(user1)がmutableなので、フィールドの値を変更できる
    // インスタンス全体が可変でなければならないことに注意。一部のフィールドのみを可変にすることはできない。
    let black = Color(0, 0, 0); // タプル構造体
    black.0; // 0

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {:#?} square pixels.", rect1.area());
}

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing);
} // ここでsome_thingがスコープを抜け、dropが呼ばれる。メモリが開放される。
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
struct Color(i32, i32, i32);

#[derive(Debug)] // デバッグ出力を可能にする
struct Rectangle {
    width: u32,
    height: u32,

}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
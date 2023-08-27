pub mod second;
pub mod third;

use second::hello;
use third::return_three::return_three;

fn main() {
    let mut x: i32 = -5;
    println!("The value Of x is: {}", x); // -5
    x = 6;
    println!("The value Of x is: {}", x); // 6

    let y: i32 = 5; // 5
    let y: i32 = y + 1; // 6
    {
        let y: i32 = y*2; // 12
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}"); // 6
    
    const CONSTANT: usize = 100; // コンパイル時に値が入っている必要がある
    println!("The value Of x is: {}", CONSTANT); // 100
    
    let some_strings: &str = "aaa";
    println!("The value Of some_strings is: {}", some_strings); // aaa
    
    let some_strings: usize = some_strings.len();
    println!("The value Of some_strings is: {}", some_strings); // 3

    // tuple
    let tup: (i32, usize, isize) = (500, 6, 1);
    let (x, y, z) = tup;
    println!("The tup value x, y, z is {}, {}, {}", x, y, z);
    println!("The tup value x.0 is {}", tup.0);
    
    let a_array: [usize; 5] = [3; 5];
    println!("The tup value a_array is {}", a_array.len());
    
    let func_return = another_function(5);
    println!("{}", func_return);
    println!("{}", plus_one(5));
    if_state(4);
    let condition = true;
    let num_if = if condition { 5 } else { 6 };
    println!("num_if is {}", num_if);
    loop_st();
    while_st();
    for_st();
    for_array_st();
    own();
    move_test();
    hello();
    println!("return {}", return_three());
}

fn another_function(x: i32) -> i32 {
    let y = {
        let z = x  +1;
        z + 1
    };
    return y
}

// セミコロンを x+ 1につけると式から文に変わるのでコンパイルエラーになる
fn plus_one (x: i32) -> i32 {
    x + 1
}

fn if_state(number: usize) {
    if number > 5  {
        println!("number was over 5");
    } else if number % 5 == 0 {
        println!("number was True");
    } else {
        println!("number was false");
    }
}

fn loop_st() {
    let mut count: usize = 0;
    'counting_up: loop {
        println!("LOOP");
        
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_st() {
    let mut number: usize = 3;

    while number != 0  {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!");
}

fn for_st() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
}

fn for_array_st() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {}", element);
    }
}
// ownership
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
fn own() {
    let mut s: String = String::from("Hello");
    s.push_str(", Ownership");

    println!("{}", s);

}
// Rustではブロックを抜けるタイミングでOSにメモリを返している

// move
fn move_test() {
    let s1 = String::from("Hello");
    // 新しいヒープ領域に新しくコピーしている→パフォーマンス悪い
    let s2 = s1;
    // println!("{}", s1); s1はもうアクセスできなくなっている
    // Cloneを使えばs1も使えるようになる
    let s3 = s2.clone();
    println!("{} {}", s2, s3); 
}

// 参照...所有権を一時的に借りる
// 借用...関数の引数に参照を受け取ること
// #[derive(Debug)]
struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}
impl User {
    // static method
    fn square() -> Self{
        User {
            username: String::from("Yamamoto"),
            email: String::from("sample@gmail.com"),
            sing_in_count: 1,
            active: true
        }
    }
}

fn struct_sample(email: String, username: String) -> User {
    let user: User = User::square();
    User {
        username,
        email,
        sing_in_count: 1,
        active: true
    }
}

enum IpAddrKind {
    v4,
    v6,
}
enum IpAddr {
    v4(String),
    c6(String),
}
fn enum_test() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
}

enum Message {
    Quit,
    Mover {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Red,
    Green,
    Blue,
}
fn color_to_str(color: &Color) -> &str {
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
    }
}
fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("found {}", number),
        None => println!("nothing found"),
    }
}
fn if_let() {
    let maybe_number: Option<u32> = Some(6);
    if let Some(number) = maybe_number {
        println!("number: {}", number);
    } else {
        println!("this is else statement");
    }
}
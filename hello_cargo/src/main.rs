fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    another_function(10);
    let five = five();
    println!("{}", five);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x)
}

fn five() -> i32 {
     5
}

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
    
    
}

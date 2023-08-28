fn largest<T>(list: &[T]) -> T {
    let mut largest = numbers_list[0];
    
    for &number in numbers_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    let numbers_list: Vec<i32> = vec![34, 50, 25, 100];
    println!("The largest number is {}", largest(&numbers_list));

    let numbers_list_i: Vec<i32> = vec![100.9, 34.1, 4342.1, 43.4, 23.3, 3.2, 3.1];
    println!("The largest number is {}", largest(&numbers_list_i));
}

// enum, struct, fn, (Result), impl でジェネリクスが使える

// クレイト境界...
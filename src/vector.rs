// Collection
// Vector...可変長の値を並べて保持する
fn vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    // ! ... マクロ（引数が複数与えられる関数のようなもの）
    let v1 = vec![1,2,3];
    match v1.get(2) {
        Some(3) => println("THIRD"),
        None => println("NONE"),
    }
}
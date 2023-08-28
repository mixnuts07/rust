// Rustのエラー...回復可能、回復不能
fn error_handling() {
    // 回復不能
    panic!("Crash and Burn"); // 強制終了したい時に使う
    // 回復可能(Result)
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}   

fn read_username_from_file() -> Result<String, io:Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// ?演算子を使ってエラー委譲を簡単に書く
fn read_username_from_file_hatena() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // File::open("hello.txt")?.read_to_string(&mut s)?;
}
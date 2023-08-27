// Rustのエラー...回復可能、回復不能
fn error_handling() {
    // 回復不能
    panic!("Crash and Burn"); // 強制終了したい時に使う
    // 回復可能(Result)
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
}   

// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let number = 3;
    call_me(number);
}

fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..3 {
        println!("Ring! Call number {}", i);
    }
    for i in 5..7 {
        println!("Ring! Call number {}", i);
    }

    let array = [1, 2, 3, 4, 5]; 
    let slice = &array[0..3];
    for i in slice {
        println!("Ring! Call number {}", i);
    }
}

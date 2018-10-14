#[no_mangle]
pub extern fn add_numbers(num1: i32, num2: i32) -> i32 {
    println!("Hello from rust");
    num1 + num2
}

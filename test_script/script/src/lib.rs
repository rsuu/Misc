#[no_mangle]
pub fn hello() {
    println!("HELLO");
}

#[no_mangle]
pub fn add_one(v: u32) -> u32 {
    1 + v
}

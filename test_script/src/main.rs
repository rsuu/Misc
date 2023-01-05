use dlopen2::wrapper::{Container, WrapperApi};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    unsafe {
        if let Ok(mut cont) = Container::<Api>::load("./lib64.so") {
            cont.hello();
            let v = cont.add_one(7);
            println!("{}", v);
        } else {
            panic!()
        }
    }

    // unsafe { cont.example_c_fun() };
    // *cont.example_reference_mut() = 5;
}

#[derive(WrapperApi)]
pub struct Api {
    hello: fn(),
    add_one: fn(v: u32) -> u32,
    //c_fun: unsafe extern "C" fn(),
    //ref_val: &'a mut i32,
}

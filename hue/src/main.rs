extern crate libc;

extern {
    fn test(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { test(input) };
    println!("test executed, with input {} output {}", input, output);
}

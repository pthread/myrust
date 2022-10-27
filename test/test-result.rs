use std::process;

fn main() {
    let x: Result<i32, &str> = Err("hi error");
    if x.is_err() {
        println!("error: {:?}", x.err());
    }

    let y = x.unwrap_or_else(|err| {println!("{:?}", err); process::exit(1);});
    println!("y={}", y);
}

use std::process;
use std::fmt;

#[derive(Debug)]
struct Config {
    restr: String,
    file: String,
}

impl Config {
    fn new(r: String, f: String) -> Result<Config, &'static str> {
        if r.len() > 10 {
            return Err("r: len > 10")
        }
        Ok(Config{ restr: r, file: f })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.restr, self.file)
    }
}

fn main() {
    let config:Config = Config::new("hello".to_string(), "world.txt".to_string())
        .unwrap_or_else(|err| {println!("{:?}", err); process::exit(1);});
    println!("config={config}");

    let x: Result<i32, &str> = Err("hi error");
    if x.is_err() {
        println!("error: {:?}", x.err());
    }

    let y = x.unwrap_or_else(|err| {println!("{:?}", err); process::exit(1);});
    println!("y={}", y);
}

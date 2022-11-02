// implement struct println with fmt::Display
// implement List println with fmt::Display
// Derive(Debug) for {:?}
// return Result
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

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vect = &self.0;
        let vect_iter = vect.iter();
        write!(f, "[")?;
        for (count, val) in vect_iter.enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", count, val)?;
        }
        write!(f, "]")
    }
}


fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    
    let config:Config = Config::new("hello".to_string(), "world.txt".to_string())
        .unwrap_or_else(|err| {println!("{:?}", err); process::exit(1);});
    println!("config={config}");
    println!("config={:?}", config);

    let x: Result<i32, &str> = Err("hi error");
    if x.is_err() {
        println!("error: {:?}", x.err());
    }

    let y = x.unwrap_or_else(|err| {println!("{:?}", err); process::exit(1);});
    println!("y={}", y);
}

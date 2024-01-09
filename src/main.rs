use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Parse Error"),
            MyError::IOError => write!(f, "IO Error"),
        }
    }
}

impl std::error::Error for MyError {}

fn square(val: &str) -> Result<i32, ParseIntError> {
    match val.parse::<i32>() {
        Ok(num) => Ok(i32::pow(num, 2)),
        Err(e) => Err(e)
    }
}

fn square_with_operator(val: &str) -> Result<i32, MyError> {
    let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;
    let mut f = File::open("fictionalfile.txt").map_err(|_| MyError::IOError)?;
    let string_to_write = format!("Square of {:?} is {:?}", num, i32::pow(num, 2));
    f.write_all(string_to_write.as_bytes()).map_err(|_| MyError::IOError)?;
    Ok(i32::pow(num, 2))
}

fn main() {
    println!("{:?}", square("2"));
    println!("{:?}", square("INVALID"));

    println!("{:?}", square_with_operator("2"));
   let result = square_with_operator("invalid");
    match result {
        Ok(res) => println!("Result is {:?}", res),
        Err(e) => println!("Error in parsing: {:?}", e)
    }
}

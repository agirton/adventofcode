use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

fn get_floors() -> Result<String> {
    let mut f = try!(File::open("./src/input.txt")
      .map_err(|err| err.to_string())
    );
    let mut floors = String::new();

    try!(f.read_to_string(&mut floors)
      .map_err(|err| err.to_string())
    );

    Ok(floors)
}

fn move_floors(floors: String) -> i32 {
    floors.chars().fold(0, |acc, item|
        match item {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        }
    )
}

fn main() {
    match get_floors() {
        Ok(n)  => println!("Floors {}", move_floors(n)),
        Err(e) => println!("Error {}", e),
    };
}

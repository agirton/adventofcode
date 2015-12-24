use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

const UP:char = '(';
const DOWN:char = ')';

fn get_floors(path: &str) -> Result<String> {
    let mut f = try!(File::open(&path)
      .map_err(|err| err.to_string())
    );
    let mut floors = String::new();

    try!(f.read_to_string(&mut floors)
      .map_err(|err| err.to_string())
    );

    Ok(floors)
}

fn increase(n: i32) -> i32 {
    n + 1
}

fn decrease(n: i32) -> i32 {
    n - 1
}

fn update_floor(floor: (i32, i32), updater: &Fn(i32) -> i32) -> (i32, i32) {
    let (acc, idx) = floor;
    (updater(acc), idx + 1)
}

fn find_last_floor(floor: Option<(i32, i32)>) -> i32 {
    match floor {
        Some(f) => f.0,
        None => 0
    }
}

fn basement_floor(floor: Option<usize>) -> usize {
    match floor {
        Some(f) => f,
        None => 0
    }
}

fn move_floors(floors: String) -> (i32, usize) {
    let mut floor = floors.chars().scan((0, 1), |fac, item|
        match item {
            UP => {
                *fac = update_floor(*fac, &increase);
                Some(*fac)
            },
            DOWN => {
                *fac = update_floor(*fac, &decrease);
                Some(*fac)
            },
            _ => Some(*fac)
        }
    );

    let first_basement_floor = floor.position(|x| x.0 == -1);
    (find_last_floor(floor.last()), basement_floor(first_basement_floor) + 1)
}

fn main() {
    match get_floors("./src/input.txt") {
        Ok(n)  => println!("Floor results {:?}", move_floors(n)),
        Err(e) => println!("Error {}", e)
    };
}

#[test]
fn test_floor() {
    match get_floors("./src/test-input.txt") {
        Ok(n) => {
            let (_, position) = move_floors(n);
            assert_eq!(position, 5);
        },
        Err(e) => println!("Error {}", e)
    }

}

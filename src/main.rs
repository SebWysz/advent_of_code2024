pub mod day_code;

use day_code::day1;
use day_code::day2;

fn main() {
    //let res = day1::first();
    //let res = day1::second();
    //let res = day2::first();
    let res = day2::second();
    match res {
        Err(e) => println!("Error: {}", e),
        Ok(n) => println!("Success! Result: {}", n),
    }
}

use std::io;
use std::error::Error;
use std::iter::zip;

fn main() -> Result<(), Box<dyn Error>>{

    let lines = io::stdin().lines();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {

        //let mut vec2 = Vec::new();

        let line_data = line?;

        let mut iterator = line_data.split_whitespace();

        let number1: i32 = iterator.next().unwrap().parse()?;
        let number2: i32 = iterator.next().unwrap().parse()?;

        vec1.push(number1);
        vec2.push(number2)

    }

    vec1.sort();
    vec2.sort();

    let mut total = 0;
    for (a, b) in zip(vec1, vec2) {
        total += (a - b).abs();
    }

    print!("{}\n", total);

    Ok(())
}

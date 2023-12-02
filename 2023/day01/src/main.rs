use std::io::{self as io, Read};

fn main() {
    let sum = io::stdin().lines().filter_map(|line|
      line.map(|l|
        l.chars().filter(|c|
          c.is_digit(10)
        ).collect::<Vec<_>>()
      ).ok().and_then(|ds|
        if ds.len() > 1 {//need >= 2 digits
            String::from_iter(
              vec![ds.first().unwrap(), ds.last().unwrap()]
            ).parse::<i32>().ok()
        } else {
          None
        }
      )
    ).fold(0, |s, d|
      //ds for certain contains >= 2 elements here
      s + d
    );

    //dbg!(sum);
    println!("The sum is {sum}")
}
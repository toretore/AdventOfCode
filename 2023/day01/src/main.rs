use std::io::{self as io, Read};

fn main() {
    let sum = io::stdin().lines().filter_map(|line|
      line.map(|l|
        l.chars().filter_map(|c|
          c.to_digit(10)
        ).collect::<Vec<_>>()
      ).ok().and_then(|ds|
        if ds.len() > 1 {//need >= 2 digits
          Some(ds)
        } else {
          None
        }
      )
    ).fold(0, |s, ds|
      //ds for certain contains >= 2 elements here
      s + ds.first().unwrap() + ds.last().unwrap()
    );

    println!("The sum is {sum}")
}
use std::{io::{self as io}, collections::HashMap};
use fancy_regex::{Regex, Match};

#[allow(unused_variables)]
fn main() {
    let words = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let re = Regex::new(
        dbg!(format!(
            "(?=({}))",
            words.keys().map(|k| *k ).collect::<Vec<_>>().join("|")
        ).as_str())
    ).unwrap();

    let sum = io::stdin().lines().filter_map(|line|
        line.ok().map(|l|
            re.captures_iter(l.as_str()).filter_map(|cs|
                cs.ok().and_then(|cs|
                    cs.get(1).and_then(|m|
                        words.get(m.as_str())
                    )
                )
            ).collect::<Vec<_>>().into_iter().rfold((None,None), |(f, l), d|
                if let Some(_) = l {
                    (Some(d), l)
                } else {
                    (f, Some(d))
                }
            )
      ).map(|(f,l)|
        dbg!((dbg!(f).or(l).unwrap_or(&0) * 10) + dbg!(l).unwrap_or(&0))
      )
    ).sum::<i32>();

    println!("The sum is {sum}")
}
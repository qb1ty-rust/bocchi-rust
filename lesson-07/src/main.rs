#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();

    a.insert(String::from("new"), 42);
    a.insert(String::from("demo"), 100);

    let value = a.get(&String::from("new")).unwrap_or(&1);

    println!("value: {}", value);

    for (k, v) in &a {
        println!("key: {}, value: {}", k, v);
    }

    a.entry(String::from("old")).or_insert(250);

    for (k, v) in &a {
        println!("key: {}, value: {}", k, v);
    }


    let sum = a.values().sum::<u32>();

    println!("{}", sum);
}
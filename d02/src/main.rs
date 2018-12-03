use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn main() {
    // Opening the input file directly
    let f = File::open("input");
    let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!(
                    "There was a problem opening the file: {:?}",
                    error
                )
            },
    };
    
    // Counter for repeating chars#
    let mut counter:(i32, i32) = (0, 0);
    // loop ower the lines in buffer
    for line in BufReader::new(f).lines() {
        let mut map:HashMap<char, i32> = HashMap::new();
        let mut id:String;
        id = match line.as_ref().unwrap().parse() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };

        // counting chars of ID to hashmap
        for c in id.chars() {
            let count = map.entry(c).or_insert(0);
            *count +=1;
        }

        // Check HashMap and decide if we count the ID
        let mut is_2 = false;
        let mut is_3 = false;
        for val in map.values() {
            match val {
                &2 => { is_2 = true },
                &3 => { is_3 = true },
                _ => (),
            }
        }
        if is_2 { counter.0 += 1 };
        if is_3 { counter.1 += 1 };
    }
    println!("The checksum is: {}", counter.0 * counter.1);
}
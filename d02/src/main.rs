use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Function to compare 2 strings and return
/// string with letters on the same possition
/// This is implemented by using iterators
/// chars() terurns iterator ower string and we zip() it to create touple
/// filter() selects only the chars whitch are equal
/// then we map() one of two values and collect it to String.
fn get_eq_letters(s1: &String, s2: &String) -> String {
    let eq = s1
        .chars()
        .zip(s2.chars())
        .filter(|s| (s.0 == s.1))
        .map(|s| s.0)
        .collect::<String>();
    eq
}

fn main() {
    // Opening the input file directly
    let f = File::open("input");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    // Counter for repeating chars (is_2, is_3)
    let mut counter: (i32, i32) = (0, 0);

    // Part 2
    // Vector to collect boxes IDs
    let mut id_store: Vec<String> = Vec::new();

    // loop ower the lines in buffer
    for line in BufReader::new(f).lines() {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut box_id: String;
        box_id = match line.as_ref().unwrap().parse() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };

        // counting chars of ID to hashmap
        for c in box_id.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        // Check HashMap and decide if we count the ID to create checksum
        let mut is_2 = false;
        let mut is_3 = false;
        for val in map.values() {
            match val {
                &2 => is_2 = true,
                &3 => is_3 = true,
                _ => (),
            }
        }
        if is_2 {
            counter.0 += 1
        };
        if is_3 {
            counter.1 += 1
        };
        
        // Find ID whith only one difrent char
        for id in id_store.iter().as_ref() {
            let one_difrent = id.len() - 1;
            let eq = get_eq_letters(&box_id, id);
            let eq_max = eq.len();
            if eq_max == one_difrent {
                println!("Common letters are: {}", eq);
            }
        }
        id_store.push(box_id.to_string());
    }

    println!("Number of boxes: {}", id_store.len());
    println!("The checksum is: {}", counter.0 * counter.1);
}

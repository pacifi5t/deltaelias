#![allow(unused)]
use std::collections::HashMap;
use std::env;
use std::string::String;
use std::{
    fs::File,
    io::{self, prelude::*},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    match option.as_str() {
        "encode" => encode(input_path, output_path),
        "decode" => decode(input_path, output_path),
        _ => println!("Neither of 'encode' or 'decode' options found")
    }
}

fn read_from_file(path: &String) -> Result<String, io::Error> {
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    print!("{}", content);

    Ok(content)
}

fn encode(input: &String, output: &String) {
    let content = read_from_file(&input).unwrap();
    let mut map: HashMap<char, u32> = HashMap::new();

    for ch in content.chars() {
        match map.get_mut(&ch) {
            Some(existing) => *existing += 1,
            None => {
                map.insert(ch, 1);
            }
        }
    }
    //TODO: Finish encoding
}

fn decode(input: &String, output: &String) {
    //TODO: Implement decoding
}
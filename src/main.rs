#![allow(unused)]
mod encoder;

use encoder::*;
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
        _ => println!("Neither of 'encode' or 'decode' options found"),
    }
}

fn encode(input: &String, output: &String) {
    let content = read_from_file(&input).unwrap();
    let alphabet = get_alphabet(&get_char_map(&content));
    let rank_map = get_rank_map(&alphabet);
    let delta_map = get_delta_map(&get_gamma_map(&rank_map));
    let encoded_content = convert_content(&content, &rank_map, &delta_map);

    println!();
    //TODO: Output in file
}

fn decode(input: &String, output: &String) {
    //TODO: Implement decoding
}

fn read_from_file(path: &String) -> Result<String, io::Error> {
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    print!("{}", content);

    Ok(content)
}

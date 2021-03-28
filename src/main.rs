#![allow(unused)]
mod encoder;

use encoder::*;
use std::collections::HashMap;
use std::env;
use std::string::String;
use std::{
    fs::OpenOptions,
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
    write_to_file(&output, &alphabet, &encoded_content);
    
    println!();
    //FIXME: REFACTORING & POLISHING!!!!!!!
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

fn write_to_file(
    path: &String,
    alphabet: &Vec<char>,
    content: &String,
) -> Result<File, io::Error> {
    let mut file = File::create(path)?;
    file.write(&alphabet.len().to_ne_bytes()[0..4]);
    file.write(vector_to_string(alphabet).as_bytes());
    file.write(encoded_to_writable(content).as_bytes());

    Ok(file)
}

fn vector_to_string(vector: &Vec<char>) -> String {
    let mut string = String::new();

    for ch in vector {
        string.push(*ch);
    }

    string
}

fn encoded_to_writable(content: &String) -> String {
    let byte_count = content.len() / 8;
    let shortage = 8 - content.len() % 8;
    let (full, short) = content.split_at(byte_count * 8);
    let mut output_str = String::new();
    
    for i in 0..byte_count {
        let buffer = &full[i * 8..i * 8 + 8];
        let ch = usize::from_str_radix(buffer, 2).unwrap().to_ne_bytes()[0];
        output_str.push(ch as char);
    }

    let a = format!("{:0>8}", short);
    let b = usize::from_str_radix(a.as_str(), 2).unwrap();
    output_str.push(usize::from_str_radix(format!("{:0>8}", short).as_str(), 2).unwrap().to_ne_bytes()[0] as char);

    output_str
}
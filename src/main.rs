#![allow(unused)]

mod encoder;

use encoder::*;
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
    string::String,
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
    let alphabet = gen_alphabet(&gen_char_map(&content));
    let rank_map = gen_rank_map(&alphabet);
    let delta_map = gen_delta_map(&gen_gamma_map(&rank_map));
    let encoded_content = encode_content(&content, &rank_map, &delta_map);
    write_encoded_to_file(&output, &alphabet, &encoded_content);
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

mod decoder;
mod encoder;

use decoder::*;
use encoder::*;
use std::{
    env,
    fs::File,
    io::prelude::*,
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
    let content = read_from_file(&input);
    let alphabet = gen_alphabet(&gen_byte_map(&content));
    let rank_map = gen_rank_map(&alphabet);
    let delta_map = gen_delta_map(&gen_gamma_map(&rank_map));
    let encoded_content = encode_content(&content, &rank_map, &delta_map);
    write_encoded_to_file(&output, &alphabet, &encoded_content);
}

fn decode(input: &String, output: &String) {
    let content = read_from_file(&input);
    let (alphabet, exp, encoded) = parse_content(&content);
    let dedoding_map = gen_decoding_map(&alphabet);
    let decoded = decode_content(&encoded, exp, &dedoding_map);
    write_decoded_to_file(output, &decoded);
}

fn read_from_file(path: &String) -> Vec<u8> {
    let mut file = File::open(&path).unwrap();
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content);

    content
}

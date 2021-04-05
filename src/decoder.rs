use std::{collections::HashMap, fs::File, io::prelude::*, string::String};

pub fn parse_content(content: &Vec<u8>) -> (Vec<u8>, u8, Vec<u8>) {
    let mut size = content[0] as usize;
    size += 1;

    let mut alphabet: Vec<u8> = Vec::new();
    for &byte in &content[1..1 + size] {
        alphabet.push(byte);
    }

    let mut encoded: Vec<u8> = Vec::new();
    for &byte in &content[2 + size..] {
        encoded.push(byte);
    }

    (alphabet, content[1 + size], encoded)
}

pub fn gen_decoding_map(alphabet: &Vec<u8>) -> HashMap<usize, u8> {
    let mut decoding_map: HashMap<usize, u8> = HashMap::new();
    for (i, &el) in alphabet.iter().enumerate() {
        decoding_map.insert(i + 1, el);
    }

    decoding_map    //K - Rank, V - Byte
}

pub fn decode_content(encoded: &Vec<u8>, ext: u8, decoding_map: &HashMap<usize, u8>) -> Vec<u8> {
    let mut encoded_str = String::new();
    for i in 0..encoded.len() {
        let mut temp = format!("{:0>8}", format!("{:b}", encoded[i]));

        if i == encoded.len() - 1 {
            temp = temp.split_off(ext as usize);
        }
        encoded_str.push_str(temp.as_str());
    }

    let vector: Vec<char> = encoded_str.chars().collect();
    let mut ranks: Vec<usize> = Vec::new();
    let mut zero_counter = 0;
    encoded_str.clear();

    let mut i = 0;
    while i < vector.len() {
        if vector[i] == '0' {
            zero_counter += 1;
            i += 1;
        } else {
            let mut buff = String::new();

            for j in i..i + zero_counter + 1 {
                buff.push(vector[j] as char);
            }
            i += zero_counter + 1;
            let useful_bits_count = usize::from_str_radix(buff.as_str(), 2).unwrap();
            buff.clear();
            buff.push('1');

            for j in i..i + useful_bits_count - 1 {
                buff.push(vector[j] as char);
            }
            i += useful_bits_count - 1;
            let rank = usize::from_str_radix(buff.as_str(), 2).unwrap();
            ranks.push(rank);
            zero_counter = 0;
        }
    }
    let mut output: Vec<u8> = Vec::new();
    for rank in ranks {
        output.push(*decoding_map.get(&rank).unwrap())
    }

    output
}

pub fn write_decoded_to_file(path: &String, decoded: &Vec<u8>) {
    let mut file = File::create(path).unwrap();
    file.write(decoded);
}

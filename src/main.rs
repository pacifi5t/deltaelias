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

fn encode(input: &String, output: &String) {
    let content = read_from_file(&input).unwrap();
    let mut char_map: HashMap<char, usize> = HashMap::new(); //K - Symbol, V - Count

    for ch in content.chars() {
        match char_map.get_mut(&ch) {
            Some(existing) => *existing += 1,
            None => {
                char_map.insert(ch, 1);
            }
        }
    }

    let rank_map = get_rank_map(&char_map);
    let gamma_map = get_gamma_map(&rank_map);
    let delta_map = get_delta_map(&gamma_map);

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

fn get_rank_map(char_map: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut vector: Vec<&char> = char_map.keys().collect();
    vector.sort_by(|a, b| char_map.get(b).unwrap().cmp(char_map.get(a).unwrap()));
    let mut rank_map: HashMap<char, usize> = HashMap::new();
    
    let mut i = 0;
    while i < vector.len() {
        rank_map.insert(*vector[i], i + 1);
        i += 1;
    };

    rank_map //K - Symbol, V - Rank
}

fn get_gamma_map(rank_map: &HashMap<char, usize>) -> HashMap<usize, String> {
    let mut gamma_map: HashMap<usize, String> = HashMap::new();
    for el in rank_map.values() {
        let zero_count = ((*el as f32).log2() as usize);
        let mut gamma_code = String::new();

        let mut i = 0;
        while i < zero_count {
            gamma_code.push('0');
            i += 1;
        }
        gamma_code.push_str(format!("{:b}", el).as_str());
        gamma_map.insert(*el, gamma_code);
    };

    gamma_map //K - Rank, V - Gamma Elias' code
}

fn get_delta_map(gamma_map: &HashMap<usize, String>) -> HashMap<usize, String> {
    let mut delta_map: HashMap<usize, String> = HashMap::new();
    for el in gamma_map.keys() {
        let temp = ((*el as f32).log2() as usize) + 1;
        let mut delta_code = String::new();
        delta_code.push_str(gamma_map.get(&temp).unwrap().as_str());
        if *el != 1 {
            delta_code.push_str(&format!("{:b}", el)[1..]);
        }
        delta_map.insert(*el, delta_code);
    };
    
    delta_map // K - Rank, V - Delta Elias' code
}
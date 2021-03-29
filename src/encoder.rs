use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*},
    string::String,
};

pub fn gen_char_map(file_content: &String) -> HashMap<char, usize> {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    for ch in file_content.chars() {
        match char_map.get_mut(&ch) {
            Some(existing) => *existing += 1,
            None => {
                char_map.insert(ch, 1);
            }
        }
    }

    char_map //K - Symbol, V - Count
}

pub fn gen_alphabet(char_map: &HashMap<char, usize>) -> Vec<char> {
    let mut alphabet: Vec<char> = Vec::new();

    for el in char_map.keys() {
        alphabet.push(*el);
    }
    alphabet.sort_by(|a, b| char_map.get(b).unwrap().cmp(char_map.get(a).unwrap()));

    alphabet
}

pub fn gen_rank_map(alphabet: &Vec<char>) -> HashMap<char, usize> {
    let mut rank_map: HashMap<char, usize> = HashMap::new();

    for (i, &el) in alphabet.iter().enumerate() {
        rank_map.insert(el, i + 1);
    }

    rank_map //K - Symbol, V - Rank
}

pub fn gen_gamma_map(rank_map: &HashMap<char, usize>) -> HashMap<usize, String> {
    let mut gamma_map: HashMap<usize, String> = HashMap::new();

    for el in rank_map.values() {
        let zero_count = (*el as f32).log2() as usize;
        let mut gamma_code = String::new();

        for _i in 0..zero_count {
            gamma_code.push('0');
        }

        gamma_code.push_str(format!("{:b}", el).as_str());
        gamma_map.insert(*el, gamma_code);
    }

    gamma_map //K - Rank, V - Gamma Elias' code
}

pub fn gen_delta_map(gamma_map: &HashMap<usize, String>) -> HashMap<usize, String> {
    let mut delta_map: HashMap<usize, String> = HashMap::new();

    for el in gamma_map.keys() {
        let temp = ((*el as f32).log2() as usize) + 1;
        let mut delta_code = String::new();

        delta_code.push_str(gamma_map.get(&temp).unwrap().as_str());
        delta_code.push_str(&format!("{:b}", el)[1..]);
        delta_map.insert(*el, delta_code);
    }

    delta_map // K - Rank, V - Delta Elias' code
}

pub fn encode_content(
    content: &String,
    rank_map: &HashMap<char, usize>,
    delta_map: &HashMap<usize, String>,
) -> String {
    let mut encoded_content = String::new();

    for ch in content.chars() {
        encoded_content.push_str(delta_map.get(rank_map.get(&ch).unwrap()).unwrap().as_str());
    }

    encoded_content
}

pub fn write_encoded_to_file(
    path: &String,
    alphabet: &Vec<char>,
    encoded: &String,
) -> Result<File, io::Error> {
    let mut file = File::create(path)?;
    file.write(&alphabet.len().to_ne_bytes()[0..4])?;
    file.write(vector_to_string(alphabet).as_bytes())?;
    file.write(encoded_to_writable(encoded).as_bytes())?;

    Ok(file)
}

pub fn vector_to_string(vector: &Vec<char>) -> String {
    let mut string = String::new();

    for ch in vector {
        string.push(*ch);
    }

    string
}

pub fn encoded_to_writable(content: &String) -> String {
    let byte_count = content.len() / 8;
    let mut shortage = 8 - content.len() % 8;
    let (full, short) = content.split_at(byte_count * 8);
    let mut output_str = String::new();

    if shortage == 8 {
        shortage = 0;
    }
    output_str.push_str(&shortage.to_string());

    for i in 0..byte_count {
        let buffer = &full[i * 8..i * 8 + 8];
        let ch = usize::from_str_radix(buffer, 2).unwrap().to_ne_bytes()[0];
        output_str.push(ch as char);
    }
    if shortage != 0 {
        output_str.push(
            usize::from_str_radix(format!("{:0>8}", short).as_str(), 2)
                .unwrap()
                .to_ne_bytes()[0] as char
        );
    }

    output_str
}

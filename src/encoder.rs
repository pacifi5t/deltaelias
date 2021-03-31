use std::{collections::HashMap, fs::File, io::prelude::*, string::String};

pub fn gen_byte_map(file_content: &Vec<u8>) -> HashMap<u8, usize> {
    let mut char_map: HashMap<u8, usize> = HashMap::new();
    for byte in file_content {
        match char_map.get_mut(&byte) {
            Some(existing) => *existing += 1,
            None => {
                char_map.insert(*byte, 1);
            }
        }
    }

    char_map //K - Byte, V - Count
}

pub fn gen_alphabet(char_map: &HashMap<u8, usize>) -> Vec<u8> {
    let mut alphabet: Vec<u8> = Vec::new();
    for el in char_map.keys() {
        alphabet.push(*el);
    }
    alphabet.sort_by(|a, b| char_map.get(b).unwrap().cmp(char_map.get(a).unwrap()));

    alphabet
}

pub fn gen_rank_map(alphabet: &Vec<u8>) -> HashMap<u8, usize> {
    let mut rank_map: HashMap<u8, usize> = HashMap::new();
    for (i, &el) in alphabet.iter().enumerate() {
        rank_map.insert(el, i + 1);
    }

    rank_map //K - Byte, V - Rank
}

pub fn gen_gamma_map(rank_map: &HashMap<u8, usize>) -> HashMap<usize, String> {
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

    gamma_map //K - Rank, V - Gamma code
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
    content: &Vec<u8>,
    rank_map: &HashMap<u8, usize>,
    delta_map: &HashMap<usize, String>,
) -> String {
    let mut encoded_content = String::new();
    for byte in content {
        encoded_content.push_str(
            delta_map
                .get(rank_map.get(&byte).unwrap())
                .unwrap()
                .as_str(),
        );
    }

    encoded_content
}

pub fn encoded_to_writable(content: &String) -> Vec<u8> {
    let byte_count = content.len() / 8;
    let mut shortage = 8 - content.len() % 8;
    let (full, short) = content.split_at(byte_count * 8);
    let mut output: Vec<u8> = Vec::new();

    if shortage == 8 {
        shortage = 0;
    };
    output.push(shortage.to_ne_bytes()[0]);

    for i in 0..byte_count {
        let buffer = &full[i * 8..i * 8 + 8];
        let ch = bin_str_to_byte(buffer);
        output.push(ch);
    }
    if shortage != 0 {
        output.push(bin_str_to_byte(format!("{:0>8}", short).as_str()));
    };

    output
}

pub fn write_encoded_to_file(path: &String, alphabet: &Vec<u8>, encoded: &String) {
    let mut file = File::create(path).unwrap();
    file.write(&(alphabet.len() - 1).to_ne_bytes()[0..1]);
    file.write(&alphabet);
    file.write(&encoded_to_writable(&encoded));
}

pub fn bin_str_to_byte(bin_str: &str) -> u8 {
    usize::from_str_radix(bin_str, 2).unwrap().to_ne_bytes()[0]
}

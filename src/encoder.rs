use std::collections::HashMap;
use std::string::String;

pub fn get_char_map(file_content: &String) -> HashMap<char, usize> {
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

pub fn get_rank_map(char_map: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut rank_map: HashMap<char, usize> = HashMap::new();
    let mut vector: Vec<&char> = char_map.keys().collect();
    vector.sort_by(|a, b| char_map.get(b).unwrap().cmp(char_map.get(a).unwrap()));
    
    for (i, &el) in vector.iter().enumerate() {
        rank_map.insert(*el, i + 1);
    }

    rank_map //K - Symbol, V - Rank
}

pub fn get_gamma_map(rank_map: &HashMap<char, usize>) -> HashMap<usize, String> {
    let mut gamma_map: HashMap<usize, String> = HashMap::new();

    for el in rank_map.values() {
        let zero_count = ((*el as f32).log2() as usize);
        let mut gamma_code = String::new();

        for i in 0..zero_count {
            gamma_code.push('0');
        }

        gamma_code.push_str(format!("{:b}", el).as_str());
        gamma_map.insert(*el, gamma_code);
    }

    gamma_map //K - Rank, V - Gamma Elias' code
}

pub fn get_delta_map(gamma_map: &HashMap<usize, String>) -> HashMap<usize, String> {
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

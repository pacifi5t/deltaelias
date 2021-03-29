pub fn parse_content(content: &Vec<u8>) -> (usize, Vec<u8>, u8, Vec<u8>) {
    let mut size = content[0] as usize;
    let mut alphabet: Vec<u8> = Vec::new();

    for &byte in (&content[1..1 + size]) {
        alphabet.push(byte);
    }

    let mut encoded: Vec<u8> = Vec::new();

    for &byte in (&content[2 + size..]) {
        encoded.push(byte);
    }
    
    (size, alphabet, content[1 + size], encoded)
}
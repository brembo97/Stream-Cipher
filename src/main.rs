fn main() {
    //TO DO: change alphabet for ASCII numeric values
    // Create col shuffle function
    let alphabet:[[char;2];13] = [
        ['a','b'],
        ['c','d'],
        ['e','f'],
        ['g','h'],
        ['i','j'],
        ['k','l'],
        ['m','n'],
        ['o','p'],
        ['q','r'],
        ['s','t'],
        ['u','v'],
        ['w','x'],
        ['y','z']
    ];
    let seed = "092810";
    let keystream = alphabet_shuffle(&seed, alphabet.clone());

    let message = "heuristicaysolocomidadelabuena";
    let encrypted_message = stream_cipher(message, &alphabet, &keystream);

    assert_eq!(message.len(), encrypted_message.len());
    println!("{}", encrypted_message);
}

fn stream_cipher(message: &str, alphabet: &[[char;2];13], keystream: &[[char;2];13]) -> String{

    let mut result = String::from("");

    // Encrypt every character
    for c in message.chars(){
        let index = find_letter(&c, alphabet);
        let key_char = keystream[index.0][index.1];

        //Perform XOR
        let xor_bytes = xor(c.to_string().as_bytes(), key_char.to_string().as_bytes());
        let new_char = String::from_utf8(xor_bytes).unwrap();
        result.push_str(&new_char);
    }
    result
}

fn xor(bytes_1:&[u8], bytes_2:&[u8]) -> Vec<u8>{
    let xor_bytes = bytes_1
        .iter()
        .zip(bytes_2.iter())
        .map(|(&x1, &x2)| (x1 ^ x2))
        .collect();

    xor_bytes
}

fn alphabet_shuffle(seed: &str, mut alphabet: [[char;2]; 13]) -> [[char;2];13]{
    assert_eq!(seed.len(), 6);

    // Parse seed
    let mut digits:Vec<usize> = vec![];
    for digit in seed.chars(){
        digits.push(digit.to_digit(10).unwrap() as usize);
    }
    // Shuffle operations
    row_swap(digits[0],digits[1], &mut alphabet);
    row_shuffle(digits[1], digits[2], &mut alphabet);
    col_swap(digits[3], digits[4], &mut alphabet);

    alphabet
}

fn row_swap(origin_index: usize, new_index: usize, alphabet: &mut [[char; 2]; 13]){
    let tmp = alphabet[origin_index];
    alphabet[origin_index] = alphabet[new_index];
    alphabet[new_index] = tmp;
}

fn row_shuffle(n_shifts: usize, n_row: usize, alphabet: &mut [[char;2];13]){

    let mut row =  alphabet[n_row];

    let mut increment = 1;
    for _ in 0..n_shifts{
        row.swap(0, increment);
        increment += 1;
    }
}

fn col_swap(origin_index: usize, new_index: usize, alphabet: &mut [[char;2];13]){

    for row in alphabet.iter_mut(){
        let tmp = row[origin_index];
        row[origin_index] = row[new_index];
        row[new_index] = tmp;
    }
}

fn find_letter(character: &char, alphabet: &[[char;2];13]) -> (usize, usize){

    for (i, &item) in alphabet.iter().enumerate(){
        if item[0] == *character{
            return (i, 0 as usize)
        }else if item[1] == *character{
            return (i, 1 as usize)
        }
    }
    (100,100)
}

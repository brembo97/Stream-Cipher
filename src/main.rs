fn main() {
    //TO DO: change alphabet for ASCII numeric values
    // Create col shuffle function
    // let alphabet:[[char;2];13] = [
    //     ['a','b'],
    //     ['c','d'],
    //     ['e','f'],
    //     ['g','h'],
    //     ['i','j'],
    //     ['k','l'],
    //     ['m','n'],
    //     ['o','p'],
    //     ['q','r'],
    //     ['s','t'],
    //     ['u','v'],
    //     ['w','x'],
    //     ['y','z']
    // ];
    let alphabet:[[u8;10];12] = [
        [27, 28, 29, 30, 31, 32, 33, 34, 35, 36],
        [37, 38, 39, 40, 41, 42, 43, 44, 45, 46],
        [47, 48, 49, 50, 51, 52, 53, 54, 55, 56],
        [57, 58, 59, 60, 61, 62, 63, 64, 65, 66],
        [47, 48, 49, 50, 51, 52, 53, 54, 55, 56],
        [57, 58, 59, 60, 61, 62, 63, 64, 65, 66],
        [67, 68, 69, 70, 71, 72, 73, 74, 75, 76],
        [77, 78, 79, 80, 81, 82, 83, 84, 85, 86],
        [87, 88, 89, 90, 91, 92, 93, 94, 95, 96],
        [97, 98, 99, 100, 101, 102, 103, 104, 105, 106],
        [107, 108, 109, 110, 111, 112, 113, 114, 115, 116],
        [117, 118, 119, 120, 121, 122, 123, 124, 125, 126]
    ];

    let seed = "143593";
    let keystream = alphabet_shuffle(&seed, alphabet.clone());

    let message = String::from("Soy Ivan, estudiante de bachillerato en un instituto de Madrid. Hace una semana estuve de viaje en Sevilla con unos amigos. Teniamos muchas ganas de ir porque nos gusta mucho el sur de Espana. La gente es muy amable y hospitalaria. Sevilla es una de las ciudades mas bonitas de Espana y de Europa. Sus monumentos mas famosos son la Giralda, La Catedral y la Torre del Oro. La Giralda esta situada junto a la Catedral. Es una torre muy alta, y sus alrededores siempre estan llenos de turistas que toman fotografias.");
    let encrypted_message = stream_cipher(&message, &keystream);
    let decrypted_message = stream_cipher(&encrypted_message, &keystream);

    assert_eq!(&message.len(), &encrypted_message.len());
    println!("{}", &encrypted_message);
    println!("{}", &decrypted_message);
    assert_eq!(&message, &decrypted_message);
}

fn stream_cipher(message: &str, keystream: &[[u8;10];12]) -> String{

    let mut result = String::from("");
    let row_size = keystream.len();
    let col_size = keystream[0].len();
    // Encrypt every character
    for (i, c) in message.chars().enumerate(){
        let key_char = keystream[i % row_size][i % col_size] as char;

        //Perform XOR
        let xor_bytes = xor(c.to_string().as_bytes(), key_char.to_string().as_bytes());
        let new_char = String::from_utf8(xor_bytes).unwrap();
        result.push_str(&new_char);
    }
    result
}
//
// fn decrypt(cyphertext: &str, keystream: &[[u8;10];12], alphabet: &[[u8;10];12]){
//
// }

fn xor(bytes_1:&[u8], bytes_2:&[u8]) -> Vec<u8>{
    let xor_bytes = bytes_1
        .iter()
        .zip(bytes_2.iter())
        .map(|(&x1, &x2)| (x1 ^ x2))
        .collect();

    xor_bytes
}

fn alphabet_shuffle(seed: &str, mut alphabet: [[u8;10]; 12]) -> [[u8;10];12]{
    assert_eq!(seed.len(), 6);

    // Parse seed
    let mut digits:Vec<usize> = vec![];
    for digit in seed.chars(){
        digits.push(digit.to_digit(10).unwrap() as usize);
    }
    // Shuffle operations
    row_swap(digits[0],digits[1], &mut alphabet);
    row_shuffle(digits[2], digits[3], &mut alphabet);
    col_swap(digits[4], digits[5], &mut alphabet);

    alphabet
}

fn row_swap(origin_index: usize, new_index: usize, alphabet: &mut [[u8; 10]; 12]){
    let tmp = alphabet[origin_index];
    alphabet[origin_index] = alphabet[new_index];
    alphabet[new_index] = tmp;
}

fn row_shuffle(n_shifts: usize, n_row: usize, alphabet: &mut [[u8;10];12]){

    let mut row =  alphabet[n_row];

    let mut increment = 1;
    for _ in 0..n_shifts{
        row.swap(0, increment);
        increment += 1;
    }
}

fn col_swap(origin_index: usize, new_index: usize, alphabet: &mut [[u8;10];12]){

    for row in alphabet.iter_mut(){
        let tmp = row[origin_index];
        row[origin_index] = row[new_index];
        row[new_index] = tmp;
    }
}

// fn find_letter(character: &char, alphabet: &[[char;2];13]) -> (usize, usize){
//
//     for (i, &item) in alphabet.iter().enumerate(){
//         if item[0] == *character{
//             return (i, 0 as usize)
//         }else if item[1] == *character{
//             return (i, 1 as usize)
//         }
//     }
//     (100,100)
// }

// fn find_u8(character: &char, alphabet: &[[u8;10];12]) -> (usize, usize){
//
//     let mut index= (100, 100);
//     for(i, &row) in alphabet.iter().enumerate(){
//         for (j, &code) in row.iter().enumerate() {
//             if *character == code as char{
//                 index = (i, j);
//                 break;
//             }
//         }
//     }
//     index
// }

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    collections::HashMap,
};

static ALPHABET:&str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn main() {
    let lines = lines_from_file("data.txt").expect("Could not load lines");
    let mut hmap:HashMap<char, i32> = HashMap::new();
    let key:&str = &lines[0];
    create_hash_map(&mut hmap);
    let mut ascii_key:i32 = get_ascii_key(key);
    for i in 1..lines.len()  {
        decode_message(&lines[i], &mut ascii_key, &mut hmap);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    return BufReader::new(File::open(filename)?).lines().collect();
}


fn create_hash_map(hmap:&mut HashMap<char, i32>){
    let mut i:i32 = 0;
    for ch in ALPHABET.chars(){
        i+=1;
        hmap.insert(ch, i);
    }
}

fn get_ascii_key(key:&str) -> i32{
    let mut ascii_key:i32 = 0;
    for ch in key.chars(){
         ascii_key += ch as i32;
    }
    return  ascii_key;
}

fn decode_message(msg:&str, ascii_key:&mut i32, hmap:&mut HashMap<char, i32>){
    let mut decoded_msg:String = String::new();
    let mut decoded_char:char;
    let mut index:i32;
    let mut last:i32 = 0;
    for ch in msg.chars(){
        if hmap.get(&ch).is_some() {
            index = (hmap.get(&ch).unwrap()-last-(*ascii_key)).rem_euclid(ALPHABET.chars().count() as i32)-1;
            decoded_char = ALPHABET.chars().nth(index as usize).unwrap();
            decoded_msg.push(decoded_char);
            last = hmap.get(&ch).unwrap()-1;
        }else{
            decoded_msg.push(ch);
        }
    }
    println!("{:?}", decoded_msg);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_ascii_key() {
        assert_eq!(get_ascii_key(&"Marvolo"), 736);
    }

    #[test]
    fn test_incorrect_ascii_key() {
        assert_ne!(get_ascii_key(&"Marvolo"), 522);
    }
}
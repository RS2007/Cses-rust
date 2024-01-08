use std::{io, cmp::max};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read from stdin");
    let char_array = input.chars().collect::<Vec<char>>();
    let mut l: u64 = 0;
    let mut r: u64 = 1;
    match char_array.len() {
        1 => {
            println!("1")
        }
        2 => {
            if char_array[0] == char_array[1] {
                println!("2");
            }else{
                println!("1");
            }
        }
        _ => {
            let mut max_till_now = 1;
            while l < char_array.len() as u64 && r < char_array.len() as u64 {
                if char_array[l as usize] == char_array[r as usize] {
                    r+=1;
                    max_till_now = max(max_till_now,r-l);
                } else {
                    l+=1;
                }
            }
            println!("{max_till_now}");
        }
    }
}

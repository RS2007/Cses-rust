use std::io;

pub fn main() {
    let mut input = String::new();
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read from stdin");
    let n = input.trim().parse::<u64>().unwrap();
    io::stdin()
        .read_line(&mut input2)
        .expect("Cannot read from stdin");
    let mut arr = input2
        .trim()
        .split(" ")
        .map(|string| string.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    assert!(arr.len() as u64 == n - 1);
    arr.sort();
    let mut found = false;

    for i in 0..arr.len() {
        if arr[i] != i as u64 + 1 {
            println!("{}", i + 1);
            found = true;
            break;
        }
    }
    if !found {
        println!("{}", n);
    }
}

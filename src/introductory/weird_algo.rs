use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read from stdin");
    let mut num = input.trim().parse::<u64>().unwrap();
    while num != 1 {
        print!("{num} ");
        num = match num % 2 {
            0 => num / 2,
            1 => num * 3 + 1,
            _ => {
                assert!(false, "Unreachable");
                num
            }
        };
    }
    print!("1");
}

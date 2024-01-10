
use std::io::stdin;

pub fn main(){
   let mut input = String::new();
   stdin().read_line(&mut input).expect("Cannot read stdin");
   let n = input.trim().parse::<usize>().unwrap();
   let mut vec1 = Vec::<u64>::new();
   let mut vec2 = Vec::<u64>::new();
   if n <= 3 && n > 1{
    println!("NO SOLUTION");
    return;
   }
   for val in 1..=n {
    match val%2 {
      0 => {
        vec2.push(val as u64);
      }
      1 => {
        vec1.push(val as u64);
      }
      _ => {
       unreachable!();
      }
    }
   }
   vec2.append(&mut vec1);
   for (indx,val) in vec2.iter().enumerate() {
    if indx != vec2.len() - 1 {
      print!("{val} ");
      continue;
    }
    print!("{val}");
   }

}

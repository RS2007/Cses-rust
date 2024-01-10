use std::io::stdin;

pub fn main(){
   let mut input = String::new();
   let mut input2 = String::new();
   stdin().read_line(&mut input).expect("Cannot read stdin");
   stdin().read_line(&mut input2).expect("Cannot read stdin");
   let n = input.parse::<u64>();
   let mut arr = input2.trim().split(" ").map(|val| val.parse::<u64>().unwrap()).collect::<Vec<u64>>();
   let mut i = 1;
   let mut moves = 0;
   while i < arr.len() {
     if arr[i] < arr[i-1]  {
         moves += arr[i-1]-arr[i];
         arr[i] = arr[i-1];
     }
     i+=1;
   }
   println!("{moves}");
}



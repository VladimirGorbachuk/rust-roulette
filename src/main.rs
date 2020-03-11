extern crate rand;
use crate::rand::Rng;

fn ping(){
	let mut input = String::new();
	println!("Are you sure you want to play?: (y/n)");
   	std::io::stdin().read_line(&mut input).unwrap();
   	let line = input.trim();
   	if line != "y" && line != "n"  {
   		ping();
   	}
   	if line == "n" {
   		std::process::exit(0);
   	}
}

fn main(){
   ping();
   let rng: i32 = rand::thread_rng().gen_range(0, 6);
   if rng == 5 {
   	println!("You a die!!!");
   	return;
   }
   main();
}
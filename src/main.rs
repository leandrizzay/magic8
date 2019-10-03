use std::io;
use rand::Rng;

fn main() {
	let num = rand::thread_rng().gen_range(1, 10);
	let mut ques = String::new();
	println!("\n .-'''-. The ");
	println!("/   _   \\ Classic ");
	println!("|  (8)  |  Magic ");
	println!("\\   ^   / 8 ");
	println!(" '-...-' Ball\n \nAsk your question:");
	io::stdin().read_line(&mut ques)
		.expect("error");
	match num {
        	1 => println!("Under no circumstance."),
        	2 => println!("No chance."),
        	3 => println!("Very doubtful."),
        	4 => println!("Wouldn't count on it."),
        	5 => println!("Reply hazy. Ask again."),
        	6 => println!("Most likely."),
        	7 => println!("Signs point to yes."),
        	8 => println!("It is certain."),
        	9 => println!("Most definitely."),
		_ => println!("error"),
	
	}

}



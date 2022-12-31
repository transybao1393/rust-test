use clap::Parser;
#[path = "mylib/mod2.rs"] mod mod2;
mod mod1;

//- modules
// mod mod1 {
//     pub mod child_mod1 {
//         pub fn test_mod1() {
//             println!("This is test mod 1")
//         }
//         pub fn test_mod2() {
//             println!("This is test mod 2")
//         }
//     }
//     mod child_mod2 {
//         pub fn test_mod5() {
//             println!("this is test mod 3")
//         }
//     }
// }
// use mod1::child_mod1::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Age
   #[arg(short, long, default_value_t = 0)]
   age: u8,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

#[derive(Default)]
struct User {
    name: String,

    age: u8
}

trait YoungUser {
    fn walk(&self) {}
    fn fly(&self) {}
	fn current_age(&self) {}
}

impl YoungUser for User {
	fn walk(&self) {
		println!("> {} is walking...", self.name);
	}
	fn fly(&self) {
		println!("{} is flying...", self.name)
	}
	fn current_age(&self) {
		println!("{} is user current age", self.age)
	}
}

fn main() {
   	let args = Args::parse();

	let mut user: User = Default::default();

	// for _ in 0..args.count {
	// 	print_name(&args.name);
	// 	print_age(&args.count);
	// }
	
	user.name = args.name;
	user.age = args.age;
   	
	println!("Username {}", user.name);
	user.walk();
	user.fly();
	user.current_age();

	//- testing modules
	//- inner modules
	// test_mod1();
	// test_mod2();

	//- outer modules from another files
	mod1::test_mod1();
	mod2::test_mod2();
}

fn print_name(name: &str) -> () {    
    println!("Your name {}", &name);
}

fn print_age(age: &u8) -> () {
    println!("Your age {}", &age);
}
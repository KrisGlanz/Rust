//use std::io;

//fn main() {
//	println!("Please input your temperature in Celsius");
//	let mut temp = String::new();
	
//	io::stdin().read_line(&mut temp)
//		.expect("Failed to read line");

 //               let newtemp: i32 = temp.trim().parse()
 //               .expect("please give me a number");
  //              let correcttemp = (newtemp * 9/5) + 32;
  //              println!("Your temprature in Celsius was {} your tempture in fahrenheit is {}", temp,correcttemp);
    
  //  }
            
//fn main() {
//    let s1 = String::from("hello");
//
//    let len = calculate_length(&s1);

  //  println!("The length of {} is {}.", s1,len);
//}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
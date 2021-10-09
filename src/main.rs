use std::io;

fn main() -> io::Result<()> {

   println!("Digit the first number: ");
   let mut input_first = String::new();
   io::stdin().read_line(&mut input_first).expect("failed to read from stdin");
   input_first.pop();

   println!("Digit the second number: ");
   let mut input_second = String::new();
   io::stdin().read_line(&mut input_second).expect("failed to read from stdin");
   input_second.pop();

   let mut first_number: i128 = input_first.parse().unwrap();
   let mut second_number: i128 = input_second.parse().unwrap();

   let mut result: i128 = 0;

   let operator = "+";

   let mut sentence = String::new();
   sentence = "a b c d e f h i j".parse().unwrap();
   println!(" {} ",sentence);

   match operator {
       "+" => result = sum(first_number,second_number),
       "-" => result = subtraction(first_number,second_number),
       "*" => result = multiplication(first_number,second_number),
       "/" => result = division(first_number,second_number),
       _   => println!("bla"),
   }

   println!("the result of the operation {} + {} = {} ",input_first,input_second,result);
   Ok(())
}

fn sum(x: i128, y: i128 ) -> i128 {
    let r: i128  = x + y;
    return r;
}

fn subtraction(x: i128, y: i128 ) -> i128 {
    let r: i128 = x - y;
    return r;
}

fn multiplication(x: i128, y: i128 )-> i128 {
    let r: i128 = x*y;
    return r;
}

fn division(x: i128, y: i128 )-> i128 {
    let mut r: i128 = 0;
    if 0 != y{
        r = x / y;
    }
    return r;
}

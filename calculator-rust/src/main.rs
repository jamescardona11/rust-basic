use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first = args.nth(1).unwrap();
  let opt = args.nth(0).unwrap().chars().next().unwrap();
  let second = args.nth(0).unwrap();

  let first_number: f32 = first.parse().unwrap();
  let second_number = second.parse::<f32>().unwrap();

  let result = operator(opt, first_number, second_number);
  
  println!("{:?}", output(first_number, second_number, opt, result));
}


fn operator(operator: char, first_number: f32, second_number: f32) -> f32{
  // if operator == '+' {
  //   return first_number + second_number;
  // } else if operator == '-' {
  //   return first_number - second_number;
  // }else if operator == '*' {
  //   return first_number * second_number;
  // }else if operator == '/' {
  //   return first_number / second_number;
  // }else {
  //   return 0.0;
  // }
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '*' | 'x' | 'X' => first_number * second_number,
    '/' => first_number / second_number,
    _ => panic!("Invalid operator used.")
  }
}

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
  format!("{} {} {} = {}", first_number, operator, second_number, result)
}
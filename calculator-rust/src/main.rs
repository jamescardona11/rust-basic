use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first = args.nth(1).unwrap();
  let opt = args.nth(0).unwrap().chars().next().unwrap();
  let second = args.nth(0).unwrap();

  let first_number: f32 = first.parse().unwrap();
  let second_number = second.parse::<f32>().unwrap();

  let result = operator(opt, first_number, second_number);
  
  println!("{:?}", result);
}


fn operator(operator: char, first_number: f32, second_number: f32) -> f32{
  if operator == '+' {
    return first_number + second_number;
  } else if operator == '-' {
    return first_number - second_number;
  }else if operator == '*' {
    return first_number * second_number;
  }else if operator == '/' {
    return first_number / second_number;
  }else {
    return 0.0;
  }
}


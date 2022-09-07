use std::env::{args};

fn main() {
  let mut arguments=args();
  let first = arguments.nth(1).unwrap();
  let operator = arguments.nth(0).unwrap().chars().next().unwrap();
  let second = arguments.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  let result = operate(operator, first_number, second_number);
  
  //println!("{} {} {}",first, operator, second);
  println!("{}",output(first_number, operator, second_number, result));

}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
  match operator{
  '+' => first_number + second_number,
  '-' => first_number - second_number,
  '/' => first_number / second_number,
  '*' | 'x' | 'X'=> first_number * second_number,
  _ => panic!("Invalid operator used.")
  }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String{format!("{} {} {} = {}", first_number, operator, second_number, result)}
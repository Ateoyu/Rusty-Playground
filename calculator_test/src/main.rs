use calculator::Calculator;

mod calculator;
fn main() {
    let mut calc = Calculator::new();
    calc.get_user_input();
    println!("Result: {}", calc.calculate());
}

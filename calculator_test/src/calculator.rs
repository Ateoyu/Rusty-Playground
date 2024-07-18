use std::io::stdin;

pub(crate) struct Calculator {
    num1: f32,
    num2: f32,
    operator: String,
}

impl Calculator {
    pub(crate) fn new() -> Calculator {
        Calculator {
            num1: 0.0,
            num2: 0.0,
            operator: String::new(),
        }
    }
    pub(crate) fn with_values(num1: f32, num2: f32, operator: String) -> Calculator {
        Calculator {
            num1,
            num2,
            operator,
        }
    }

    pub(crate) fn get_user_input(&mut self) {
        println!("Enter the first number: ");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        self.num1 = input.trim().parse().expect("Please type a number");

        println!("Enter the second number: ");
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read line");
        self.num2 = input.trim().parse().expect("Please type a number");

        println!("Enter the operator: ");
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read line");
        self.operator = input.trim().to_string();
    }

    pub(crate) fn calculate(&self) -> f32 {
        let result = match self.operator.as_str() {
            "+" => self.num1 + self.num2,
            "-" => self.num1 - self.num2,
            "*" => self.num1 * self.num2,
            "/" => self.num1 / self.num2,
            _ => panic!("Invalid operator"),
        };
        return result;
    }
}

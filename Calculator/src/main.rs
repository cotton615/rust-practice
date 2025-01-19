fn main() {
    println!("**** CALCULATOR ****");
    loop {
        let mut input = String::new(); // Создание пустой строки, которая потом будет заполняться вводом из консоли.

        println!("Enter first number:");
        std::io::stdin().read_line(&mut input).unwrap();

        let first_num: f64 = input.trim().parse().expect("Enter valid number"); 

        input.clear();

        println!("Enter an operator: (+; -; /; *, power)");
        std::io::stdin().read_line(&mut input).unwrap();

        let operator: String = input.to_string();
        input.clear();

        println!("Enter second number:");
        std::io::stdin().read_line(&mut input).unwrap();

        let second_num: f64 = input.trim().parse().expect("Enter valid number"); 
        input.clear();
        
        let result: f64 = match operator.trim() {
            "+" => first_num + second_num,
            "-" => first_num - second_num,
            "*" => first_num * second_num,
            "/" => {
                if second_num != 0.0 {
                    first_num / second_num
                } else {
                    println!("Can't divide by zero!");
                    return;
                }
            }
            
            "power" => first_num.powf(second_num),

            _ => {
                println!("unexpected operator. Exiting the program...");
                return;
            }
        };

        println!("{first_num} {} {second_num} = {result}", operator.trim());
        println!("********************")
    }
}

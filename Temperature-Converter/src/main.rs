fn main() {
    println!("**** TEMPERATURE CONVERTER ****");
    
    loop {
        println!("C to F / F to C? (press 1 or 2, 3 for exit)");

        let mut temp_type: String = String::new();

        std::io::stdin().read_line( &mut temp_type)
            .expect("Failed to read input");

        let temp_type: u8 = match temp_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number (1 or 2).");
                continue;  
            }
        };

        if temp_type == 1 {
            let mut temperature: String = String::new();
            println!("Enter temperature IN CELCIUS: ");

            std::io::stdin().read_line(&mut temperature).unwrap();
            
            let temperature: f64 = temperature.trim().parse().unwrap(); 

            let temperature: f64 = temperature * 9.0/5.0 + 32.0;
            println!("Celsius to Fahrenheit: {}", temperature);

        } else if temp_type == 2 {
            let mut temperature: String = String::new();
            println!("Enter temperature IN FAHRENHEIT: ");

            std::io::stdin().read_line(&mut temperature).unwrap();
            
            let temperature: f64 = temperature.trim().parse().unwrap(); 

            let temperature: f64 = (temperature - 32.0) * 5.0/9.0;

            println!("Fahrenheit to Celsius : {}", temperature);

        } else if  temp_type == 3{
            println!("exiting...");
            break;
        } else {
            println!("Enter digits only in range of 3.");
        }
    }

    println!("*******************************");
}


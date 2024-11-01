use std::io;

fn temp_c(temp_f: f64) -> f64 {
    return (temp_f - 32.0) * (5.0/9.0);
}

fn main() {
    loop {
        println!("Please input your temp in degrees fahrenheit.");

        let mut temp_in_f = String::new();
        io::stdin()
            .read_line(&mut temp_in_f)
            .expect("Failed to read line");

        let temp_in_f: f64 = match temp_in_f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{:.2}", temp_c(temp_in_f));
        break;
    }
}

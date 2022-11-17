fn is_leap_year(year: i32) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, _) => true,
        (_, 0, _) => false,
        (_, _, 0) => true,
        _ => false
    }
}

fn main() {
    use std::io::stdin;
    
    loop {
        let mut number = String::new();
        stdin().read_line(&mut number)
            .expect("Failed to read line");
        
        let number: i32 = match number.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };

        match number {
            year @ 0.. => {
                println!("{year} is leap year?\n{}.", if is_leap_year(year) { "yes"} else {"no"});
            }
            _ => {
                break;
            }
        }
    }
}
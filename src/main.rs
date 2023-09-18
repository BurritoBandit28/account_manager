mod accounts;

use std::string::ToString;
use accounts::check_password_strength;
use accounts::PasswordFeedback;


fn main() {
    let mut pass: String = String::new();
    std::io::stdin().read_line(&mut pass).expect("oh no!");
    let result : PasswordFeedback = check_password_strength(pass.trim().to_string());
    println!("{} {}", result.message, result.code);
}
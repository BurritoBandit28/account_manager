
pub struct PasswordFeedback {
    pub message : String,
    pub code : u8
}

pub fn check_password_strength(pswd: &String) -> PasswordFeedback {

    // A list of commonly used passwords
    let common_passwords : Vec<&str> = vec!["p4ssword","p4ss0rd"];

    // A list of blacklisted characters
    let char_blacklist: Vec<char> = vec!['\\', ' '];

    // Ints to give a strength score, which can be handled in different ways
    let mut has_upper: u8 = 0;
    let mut has_unique: u8 = 0;
    let mut has_number: u8 = 0;

    let mut total : u8;

    let chars: Vec<char> = pswd.chars().collect();

    // Check if a common password is being used
    if common_passwords.contains(&pswd.to_lowercase().as_str()) {
        return PasswordFeedback{
            message : "Password too easy to guess".to_string(),
            code : 0
        }
    }

    // Iterate each character and checks certain conditions
    for c in chars {
        if c.is_uppercase() {
            has_upper = 1;
        }
        if !c.is_alphanumeric() {
            has_unique = 1;
        }
        if c.is_numeric() {
            has_number = 1;
        }
        if char_blacklist.contains(&c) {
            return PasswordFeedback{
                message : "Password contains banned character".to_string(),
                code : 0
            }
        }
    }

    // Add up total score
    total = has_unique + has_number + has_upper;

    // Check password length
    if pswd.len() < 7 {
        return PasswordFeedback{
            message : "Password too short".to_string(),
            code : total
        }
    }

    // adds a score for the password being long enough
    total += 1;

    // an extra point if the password is 10 chars or longer
    if pswd.len() > 9 {
        total+=1;
    }

    if has_upper == 0 {
        return PasswordFeedback{
            message : "Password needs upper case character".to_string(),
            code : total
        }
    }
    if has_unique == 0 {
        return PasswordFeedback{
            message : "Password needs a symbol character".to_string(),
            code : total
        }
    }
    if has_number == 0 {
        return PasswordFeedback{
            message : "Password needs number character".to_string(),
            code : total
        }
    }
    else {
        return PasswordFeedback{
            message : "Password strong!".to_string(),
            code : total
        }
    }
}
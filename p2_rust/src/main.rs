extern crate custom_error;
use custom_error::custom_error;

custom_error!{InvalidSyntaxError
    Bad{pos:i32} = "Syntax error at character position {pos}"
}

struct SimpleParser {
    input: String,
    char_pos: i32,
    input_len: i32
}

impl SimpleParser {
    fn new(user_input: &str) -> SimpleParser {
        SimpleParser {
            // initialize attributes
            input: user_input.to_string(),
            char_pos: 0,
            input_len: user_input.to_string().chars().count() as i32
        }
    }

    fn fun_s(&mut self) {
        // Accounting for empty string
        if self.input_len == 0 {
            println!("Syntax error at character position {}", self.char_pos);
            return;
        }
        // Get letter at current char_pos
        let mut letter:char = self.input.chars().nth(self.char_pos as usize).unwrap();
        if letter == 'a'{
            // loop through repeating As if any
            while letter == 'a' && self.char_pos < self.input_len {
                letter = self.input.chars().nth(self.char_pos as usize).unwrap();
                self.char_pos += 1;
            }
            self.char_pos -= 1;
            // error if input string contains entirely of As
            if letter == 'a' {
                println!("Syntax error at character position {}", self.char_pos);
                return;
            } 
        } else if letter == 'b' {
            // error if only one b
            if self.char_pos == self.input_len - 1 {
                println!("Syntax error at character position {}", self.char_pos);
                return;
            } else {
                self.char_pos += 1;
            }
        } 
        
        // Catch errors that are thrown
        match self.fun_x() {
            Ok(_) => println!("Input is valid"),
            Err(e) => println!("{}", e)
        }
    }

    fn fun_x(&mut self) -> Result<(), InvalidSyntaxError> {
        let  letter:char = self.input.chars().nth(self.char_pos as usize).unwrap();
        if letter == 'c' || letter == 'd' {
            // If c/d is the last letter in input, it is valid
            if self.char_pos == self.input_len - 1 {
                return Ok(());
            } else {
                self.char_pos += 1;
                return Err(InvalidSyntaxError::Bad{pos: self.char_pos});
            }
            // Any char other than c or d
        } else {
            return Err(InvalidSyntaxError::Bad{pos: self.char_pos});
        }
    }
}


fn main() {
    let input: [&str; 6] = ["bc", "acd", "aaad", "c", "3yz", ""];
    for i in 0..6 {
        let mut sp = SimpleParser::new(&input[i]);
        sp.fun_s();
    }
    
}
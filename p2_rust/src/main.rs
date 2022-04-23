struct SimpleParser {
    input: String,
    char_pos: i32,
    input_len: i32
}

impl SimpleParser {
    fn new(user_input: &str) -> SimpleParser {
        SimpleParser {
            input: user_input.to_string(),
            char_pos: 0,
            input_len: user_input.to_string().chars().count() as i32
        }
    }

    fn fun_s(&mut self) {
        
        if self.input_len == 0 {
            println!("Syntax error at character position {}", self.char_pos);
            return;
        }
        let mut letter:char = self.input.chars().nth(self.char_pos as usize).unwrap();
        if letter == 'a'{
            while letter == 'a' && self.char_pos < self.input_len {
                letter = self.input.chars().nth(self.char_pos as usize).unwrap();
                self.char_pos += 1;
            }
            self.char_pos -= 1;
            if letter == 'a' {
                println!("Syntax error at character position {}", self.char_pos);
                return;
            } else {
                self.fun_x();
            }
        } else if letter == 'b' {
            if self.char_pos == self.input_len - 1 {
                println!("Syntax error at character position {}", self.char_pos);
                return;
            } else {
                self.char_pos += 1;
                self.fun_x();
            }
        } else {
            self.fun_x();
        }

    }

    fn fun_x(&mut self) {
        let  letter:char = self.input.chars().nth(self.char_pos as usize).unwrap();
        if letter == 'c' || letter == 'd' {
            if self.char_pos == self.input_len - 1 {
                println!("Input is valid");
                return;
            } else {
                self.char_pos += 1;
                println!("Syntax error at character position {}", self.char_pos);
                return;
            }
            
        } else {
            println!("Syntax error at character position {}", self.char_pos);
            return;
        }
    }
}


fn main() {
    let input = "aaaa";
    let mut sp = SimpleParser::new(input);
    sp.fun_s();
}

enum State {
    Normal,
    SingleQuote,
    DoubleQuote,
    Escaped,
    DoubleQuoteEscaped,
}

//so the logic is as follows:
//1. we start in Normal state, where we read characters one by one
//2. accumulate them until we hit a space, at which point we push the accumulated word to the result vector and reset it
//3. if we hit another state we switch to that state and handle characters accordingly
//4. finally we check for unclosed quotes and return the result vector

pub struct ArgumentParser {
    input: String,              //raw input string to be parsed
    mode: State,                //current parsing state
    current_word: String,        //accumulates characters for current argument
    result: Vec<String>,        //final list of parsed arguments
}

impl ArgumentParser {
    
    //cosntructor
    pub fn new(input: String) -> Self {
        ArgumentParser {
            input,
            mode: State::Normal,
            current_word: String::new(),
            result: Vec::new()
        }
    }

    //the main parser, heart of this file
    //example input: echo "Hello World" > output.txt
    //example output: ["echo", "Hello World", ">", "output.txt"]
    pub fn parse(&mut self) -> Vec<String> {
        for ch in self.input.chars() {
            match self.mode {
                State::Normal => {
                    match ch {
                        ' ' => {
                            if !self.current_word.is_empty() {                   //if we encounter space after argument 
                                self.result.push(self.current_word.clone());     //copy and push
                                self.current_word.clear();                       //reset current word
                            } else {
                                continue;                                       //ignore multiple spaces
                            }
                        }
                        '\'' => self.mode = State::SingleQuote,                     
                        '\"' => self.mode = State::DoubleQuote,                     
                        '\\' => self.mode = State::Escaped,                         
                        _ => self.current_word.push(ch),                         //accumulate character
                    }
                }

                State::SingleQuote => {
                    if ch == '\'' { self.mode = State::Normal; }
                    else { self.current_word.push(ch); }
                }

                State::DoubleQuote => {
                    if ch == '\"' { self.mode = State::Normal; }
                    else if ch == '\\' { self.mode = State::DoubleQuoteEscaped; }
                    else { self.current_word.push(ch); }
                }

                State::Escaped => {
                    self.current_word.push(ch);                                  //take next char literally
                    self.mode = State::Normal;                                  //return to normal state
                }

                State::DoubleQuoteEscaped => {
                    match ch {
                        '"' => { 
                            self.current_word.push('\"');
                            self.mode = State::DoubleQuote; 
                        }

                        '\\' => {
                            self.current_word.push('\\');
                            self.mode = State::DoubleQuote;
                        }

                        _ => {
                            self.current_word.push('\\'); //keep the backslash if it's not escaping " or \
                            self.current_word.push(ch);
                            self.mode = State::DoubleQuote;
                        }                     
                    }
                }
            }
        }
        
        //check for unclosed quotes
        if let State::SingleQuote | State::DoubleQuote = self.mode {
            eprintln!("parse error: unclosed quote");
            self.result.clear(); //clear result on error
            return self.result.clone();
        }

        //push the last argument if we ended with a non-empty current word
        if !self.current_word.is_empty() {                   
            self.result.push(self.current_word.clone());
        }
        self.result.clone()
    }
}
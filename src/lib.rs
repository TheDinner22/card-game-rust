pub mod helpers {
    use std::io::{self, Write};

    pub fn user_input(prompt: Option<&str>) -> String {
        loop {
            if let Some(msg) = prompt {
                print!("{msg}");
                io::stdout().flush().unwrap();
            }
            
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                println!("could not read from the stdin, please try again");
                continue;
            }
    
            return input.trim().to_string();
        }
    }

    pub fn restricted_user_input(prompt: Option<&str>, ok_res: Vec<&str>) -> String { // todo this is done poorly
        let ok_res_string: Vec<String> = ok_res.into_iter().map(|s| s.to_string()).collect();

        loop {
            let input = user_input(prompt);
    
            if ok_res_string.contains(&input) { return input; }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
        assert_eq!(1+1, 2);
    }
}
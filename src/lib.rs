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
        // stupid proof
        if ok_res.len() == 0 { panic!("you made an infinite loop!") }

        let ok_res_string: Vec<String> = ok_res.into_iter().map(|s| s.to_string()).collect();

        loop {
            let input = user_input(prompt);
    
            if ok_res_string.contains(&input) { return input; }
        }
    }
}

pub mod deck_trait {
    pub trait IsDeck<T> { // todo blanket implement for Vec<T> cuz it just yoinks random element
        fn random(&mut self) -> T;
    }
}

pub mod my_random {
    use rand::prelude::*;

    pub fn random() -> f64 { // random number from 0-1 inclusive
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

#[cfg(test)]
mod tests {
    use super::my_random;

    #[test]
    fn random_in_range(){
        assert!(my_random::random() >= 0.0 );
        assert!(my_random::random() <= 1.0 );
    }

    #[test]
    fn random_works(){
        for _ in 0..10000 {
            assert!(my_random::random() != my_random::random());
        }
    }
}
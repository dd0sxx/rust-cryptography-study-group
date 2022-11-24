use std::io;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    println!("Please choose from the following menu.");
    let selected = select_screen();
    let mut key = String::new();

    match selected {
        Ok(val) => {
            if val == "Encode" {
                let mut message = String::new();
                println!("Please input your message.");
                io::stdin()
                    .read_line(&mut message)
                    .expect("Failed to read line");

                println!("Please input your secret encoding key.");
                io::stdin()
                    .read_line(&mut key)
                    .expect("Failed to read line");

                let res = encode_vignere(message, key);
                println!("your ciphertext ser: {}", res.unwrap());
            } else {
                let mut ciphertext = String::new();
                println!("Please input your ciphertext.");
                io::stdin()
                    .read_line(&mut ciphertext)
                    .expect("Failed to read line");

                println!("Please input the secret decoding key.");
                io::stdin()
                    .read_line(&mut key)
                    .expect("Failed to read line");

                let res = decode_vignere(ciphertext, key);
                println!("your message ser: {}", res);
            }
        }
        Err(err) => {
            // Do something with the error if you want
            println!("Error: {}", err);
        }
    }
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn char_to_pos(input: char) -> usize {
    ALPHABET.chars().position(|c| input == c).unwrap()
}

pub fn encode_vignere(message: String, key: String) -> Result<String, &'static str> {
    let k = key.trim().to_string().to_lowercase();
    let m = message.to_lowercase();
    let mut cipher_text: String = "".to_string();
    let mut key_iteration: usize = 0;
    for message_char in m.chars() {
        if message_char == ' ' || !message_char.is_alphabetic() {
            cipher_text.push(message_char);
        } else if message_char == '\n' {
            return Ok(cipher_text);
        } else {
            let new_char_index = (char_to_pos(message_char)
                + char_to_pos(k.chars().nth(key_iteration).unwrap()))
                % 26;
            let key_length = k.chars().count() - 1;
            if key_iteration == key_length {
                key_iteration -= key_length;
            } else {
                key_iteration += 1;
            }
            let new_char = ALPHABET.chars().nth(new_char_index).unwrap();
            cipher_text.push(new_char);
        }
    }
    Ok(cipher_text)
}

pub fn decode_vignere(ciphertext: String, key: String) -> Result<String, &'static str> {
    let k = key.trim().to_string().to_lowercase();
    let c = ciphertext.to_lowercase();
    let mut message: String = "".to_string();
    let mut key_iteration: usize = 0;
    for cipher_char in c.chars() {
        if !cipher_char.is_alphabetic() {
            message.push(cipher_char);
        } else if cipher_char == '\n' {
            return Ok(message);
        } else {
            let new_char_index = (char_to_pos(cipher_char) + 26
                - char_to_pos(k.chars().nth(key_iteration).unwrap()))
                % 26;
            let key_length = k.chars().count() - 1;
            if key_iteration == key_length {
                key_iteration -= key_length;
            } else {
                key_iteration += 1;
            }
            let new_char = ALPHABET.chars().nth(new_char_index).unwrap();
            message.push(new_char);
        }
    }
    Ok(message)
}

fn select_screen() -> std::io::Result<(String)> {
    let items = vec!["Encode", "Decode"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => Ok(items[index].to_string()),
        None => Ok("".to_string()),
    }
}

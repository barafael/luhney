use luhney::{help_text, luhn};
use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut prompt = Editor::<()>::new().unwrap();
    loop {
        match prompt.readline("cc >> ") {
            Ok(line) => {
                if line == "exit" || line == "quit" {
                    break;
                }
                prompt.add_history_entry(&line);

                if line == "help" {
                    println!("{}", help_text());
                    continue;
                }

                if luhn(line.as_str()) {
                    println!("Valid");
                } else {
                    println!("Invalid");
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
}

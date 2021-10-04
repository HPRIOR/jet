use std::process;
use crate::jet_brains_app::JetBrainsApp;
use crate::execute_command::open_jetbrains_app;

pub fn display_ui(jet_brains_apps: &Vec<JetBrainsApp>, path_str: &str) {
    display_selection(jet_brains_apps);
    println!("enter app number: ");
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1)
        });
        match parse_input(buffer.as_str()) {
            Ok(input) => {
                match handle_parsed_input(&jet_brains_apps, path_str, input) {
                    Ok(_) => { break; }
                    Err(message) => { println!("{}", message); }
                }
            }
            Err(message) => {
                println!("{}", message);
            }
        };
        buffer.clear();
    }
}

fn handle_parsed_input<'a>(jet_brains_apps: &'a Vec<JetBrainsApp>, path_str: &'a str, i: u8) -> Result<(), &'a str> {
    let num_of_apps = jet_brains_apps.len() as u8;
    if i > 0 && i <= num_of_apps {
        open_jetbrains_app(path_str, &jet_brains_apps[(i - 1) as usize]);
        Ok(())
    } else {
        Err("please enter a value within the correct range")
    }
}

fn parse_input(input: &str) -> Result<u8, &str> {
    let parsed_string = input
        .to_string()
        .replace('\n', "")
        .parse();
    match parsed_string {
        Ok(result) => Ok(result),
        Err(_) => Err("Please enter a valid number")
    }
}

fn display_selection(jet_brains_apps: &Vec<JetBrainsApp>) {
    jet_brains_apps
        .into_iter()
        .enumerate()
        .for_each(|(i, app)| {
            print!("{:?} ({}) \n", app, i + 1);
        })
}

#[cfg(test)]
mod tests {
    // import names from outer scope
    mod test_parse_input {
        use crate::ui::parse_input;

        #[test]
        fn parses_single_character() {
            let sut = parse_input("1");
            assert_eq!(sut.unwrap(), 1);
        }

        #[test]
        fn parses_10() {
            let sut = parse_input("10");
            assert_eq!(sut.unwrap(), 10);
        }

        #[test]
        fn returns_message_on_error() {
            let sut = parse_input("hello");
            assert_eq!(sut.err().unwrap(), "Please enter a valid number")
        }

        #[test]
        fn will_parse_with_cr() {
            let sut = parse_input("1\n").unwrap();
            assert_eq!(sut, 1);
        }
    }
}
use std::process;

use crate::jet_brains_app::JetBrainsApp;

// return path instead
pub fn get_selected_app(jet_brains_apps: &[JetBrainsApp]) -> &JetBrainsApp {
    println!("Enter app number: ");
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1)
        });
        if let Ok(parsed_input) = parse_input(buffer.as_str()) {
            let num_of_apps = jet_brains_apps.len() as u8;
            if parsed_input > 0 && parsed_input <= num_of_apps {
                return &jet_brains_apps[(parsed_input - 1) as usize];
            }
        }
        buffer.clear();
    }
}

fn parse_input(input: &str) -> Result<u8, &str> {
    let parsed_string = input.to_string().replace('\n', "").parse();
    match parsed_string {
        Ok(result) => Ok(result),
        Err(_) => Err("Please enter a valid number"),
    }
}

#[cfg(test)]
mod tests {
    // import names from outer scope
    mod test_parse_input {
        use crate::app_choice::parse_input;

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

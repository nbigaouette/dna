extern crate toml;

// Import traits
use std::io::prelude::Read;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn test() {
    println!("Hello from lib.rs");
}

#[derive(Debug)]
enum OnSuccess {
    Continue,
    Warn {message: String},
    Abort {message: String},
}

#[derive(Debug)]
enum OnFailure {
    Continue,
    Warn {message: String},
    Abort {message: String},
}

#[derive(Debug)]
enum Step {
    Echo {name: String, string: String},
    Run {name: String, command: String, arguments: String, on_success: OnSuccess, on_failure: OnFailure},
    Shell,
    SetEnv,
}


fn parse_toml(filename: &str) -> Vec<Step> {

    println!("Reading toml file {}", filename);

    // Read toml file content
    let mut input = String::new();
    std::fs::File::open(&filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    // Pase toml file
    let mut parser = toml::Parser::new(&input);

    let toml = parser.parse().unwrap_or_else(|| {
        for err in &parser.errors {
            let (loline, locol) = parser.to_linecol(err.lo);
            let (hiline, hicol) = parser.to_linecol(err.hi);
            println!("{}:{}:{}-{}:{} error: {}",
                     filename, loline, locol, hiline, hicol, err.desc);
        }
        panic!("Error parsing toml file.");
    });

    println!("toml: {:#?}", toml);
    println!("toml: {:?}", toml.len());

    // Parse "on" table
    // unimplemented!();

    // Parse "variables" table
    // unimplemented!();

    // Parse "steps" table
    let mut steps = Vec::<Step>::with_capacity(10);

    for step in toml.get("step").unwrap().as_slice().unwrap() {
        let action = step.as_table().unwrap().get("action").unwrap().as_str().unwrap();
        let name = step.as_table().unwrap().get("name").unwrap().as_str().unwrap().to_string();
        let details = step.as_table().unwrap().get("details").unwrap().as_table().unwrap();
        let step = match action {
            "echo" => {
                let string = details.get("string").unwrap().as_str().unwrap().to_string();
                Step::Echo {name: name, string: string}
            },
            "run" => {
                let command = details.get("command").unwrap().as_str().unwrap().to_string();
                let arguments = details.get("arguments").unwrap().as_str().unwrap().to_string();

                let on_success = match details.get("on_success") {
                    None => {
                        println!("WARNING: No 'on_success' set for {:?}, default of 'OnSuccess::Continue'", name);
                        OnSuccess::Continue
                    },
                    Some(toml_value) => {
                        let table = toml_value.as_table().unwrap();
                        let (message_type, message) = table.iter().nth(0).unwrap();
                        match message_type.as_ref() {
                            "warn" => OnSuccess::Warn {message: message.as_str().unwrap().to_string()},
                            "abort" => OnSuccess::Abort {message: message.as_str().unwrap().to_string()},
                            _ => unimplemented!()
                        }
                    }
                };

                let on_failure = match details.get("on_failure") {
                    None => {
                        println!("WARNING: No 'on_failure' set for {:?}, default of 'OnFailure::Continue'", name);
                        OnFailure::Continue
                    },
                    Some(toml_value) => {
                        let table = toml_value.as_table().unwrap();
                        let (message_type, message) = table.iter().nth(0).unwrap();
                        match message_type.as_ref() {
                            "warn" => OnFailure::Warn {message: message.as_str().unwrap().to_string()},
                            "abort" => OnFailure::Abort {message: message.as_str().unwrap().to_string()},
                            _ => unimplemented!()
                        }
                    }
                };

                Step::Run {name: name, command: command, arguments: arguments, on_success: on_success, on_failure: on_failure}
            },
            // "shell" => {
            // },
            // "setenv" => {
            // },
            _ => {
                println!("Unknown action '{}' in toml file", action);
                unimplemented!();
            },
        };
        println!("Step: {:?}", step);

        steps.push(step);
    }

    steps
}

pub fn execute_steps(filename: &str) {

    let steps = parse_toml(filename);

    println!("Steps: {:#?}", steps);
}

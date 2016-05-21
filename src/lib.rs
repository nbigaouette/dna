extern crate toml;
extern crate shellexpand;

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
    Echo {message: String},
    Warn {message: String},
    Abort {message: String},
}

#[derive(Debug)]
enum OnFailure {
    Continue,
    Echo {message: String},
    Warn {message: String},
    Abort {message: String},
}

#[derive(Debug)]
enum Pipe {
    Null,
    StdOut,
    StdErr,
    Variable {name: String},
    File {filename: String},
}

#[derive(Debug)]
enum Step {
    Echo {name: String, message: String},
    Run {name: String, command: String, arguments: String, on_success: OnSuccess, on_failure: OnFailure},
    Shell {name: String, command: String, stdout: Pipe, on_success: OnSuccess, on_failure: OnFailure},
    SetEnv {variable: String, value: String},
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

    // println!("toml: {:#?}", toml);

    // Parse "variables" table
    for (variable, value) in toml.get("variables").unwrap().as_table().unwrap() {
        let value = value.as_str().unwrap();
        // println!("variable: {}   value: {}", variable, value);
        std::env::set_var(variable, value);
    }

    // Parse "steps" table
    let mut steps = Vec::<Step>::with_capacity(10);

    for step in toml.get("step").unwrap().as_slice().unwrap() {
        let action = step.as_table().unwrap().get("action").unwrap().as_str().unwrap();
        let name = step.as_table().unwrap().get("name").unwrap().as_str().unwrap().to_string();
        let details = step.as_table().unwrap().get("details").unwrap().as_table().unwrap();
        let step = match action {
            "echo" => {
                let message = details.get("message").unwrap().as_str().unwrap().to_string();
                Step::Echo {name: name, message: message}
            },
            "run" => {
                let command = details.get("command").unwrap().as_str().unwrap().to_string();

                // Allow non-existing 'arguments' field, defaulting to empty string.
                let arguments_option = details.get("arguments");
                let arguments = match arguments_option {
                    None => String::new(),
                    Some(arguments_toml_value) => arguments_toml_value.as_str().unwrap().to_string(),
                };

                let on_success = match details.get("on_success") {
                    None => {
                        println!("WARNING: No 'on_success' set for {:?}, default of 'OnSuccess::Continue'", name);
                        OnSuccess::Continue
                    },
                    Some(toml_value) => {
                        let table = toml_value.as_table().unwrap();
                        let (message_type, message) = table.iter().nth(0).unwrap();
                        match message_type.as_ref() {
                            "echo" => OnSuccess::Echo {message: message.as_str().unwrap().to_string()},
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
                            "echo" => OnFailure::Echo {message: message.as_str().unwrap().to_string()},
                            "warn" => OnFailure::Warn {message: message.as_str().unwrap().to_string()},
                            "abort" => OnFailure::Abort {message: message.as_str().unwrap().to_string()},
                            _ => unimplemented!()
                        }
                    }
                };

                Step::Run {name: name, command: command, arguments: arguments, on_success: on_success, on_failure: on_failure}
            },
            "setenv" => {
                let variable = details.get("variable").unwrap().as_str().unwrap().to_string();
                let value = details.get("value").unwrap().as_str().unwrap().to_string();
                Step::SetEnv {variable: variable, value: value}
            },
            "shell" => {
                let command = details.get("command").unwrap().as_str().unwrap().to_string();

                let stdout = match details.get("stdout") {
                    None => {
                        Pipe::StdOut
                    },
                    Some(toml_value) => {
                        Pipe::Variable {name: toml_value.as_str().unwrap().to_owned()}
                    }
                };

                let on_success = match details.get("on_success") {
                    None => {
                        println!("WARNING: No 'on_success' set for {:?}, default of 'OnSuccess::Continue'", name);
                        OnSuccess::Continue
                    },
                    Some(toml_value) => {
                        let table = toml_value.as_table().unwrap();
                        let (message_type, message) = table.iter().nth(0).unwrap();
                        match message_type.as_ref() {
                            "echo" => OnSuccess::Echo {message: message.as_str().unwrap().to_string()},
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
                            "echo" => OnFailure::Echo {message: message.as_str().unwrap().to_string()},
                            "warn" => OnFailure::Warn {message: message.as_str().unwrap().to_string()},
                            "abort" => OnFailure::Abort {message: message.as_str().unwrap().to_string()},
                            _ => unimplemented!()
                        }
                    }
                };

                Step::Shell {name: name, command: command, stdout: stdout, on_success: on_success, on_failure: on_failure}
            },
            _ => {
                println!("Unknown action '{}' in toml file", action);
                unimplemented!();
            },
        };
        // println!("Step: {:?}", step);

        steps.push(step);
    }

    steps
}

pub fn execute_steps(filename: &str) {

    let steps = parse_toml(filename);

    println!("Steps: {:#?}", steps);

    // Loop over steps
    let nb_steps = steps.len();
    for (idx, step) in steps.into_iter().enumerate() {
        // println!("Step {}/{} -- ", idx+1, nb_steps);
        match step {
            Step::Echo {name, message} => {
                // NOTE: If an environment variable is non-existing, the following line will panic.
                println!("{}", shellexpand::full(&message).unwrap());
            },
            Step::Run {name, command, arguments, on_success, on_failure} => {
                // NOTE: If an environment variable is non-existing, the following line will panic.
                let command: String = shellexpand::full(&command).unwrap().to_string();
                // NOTE: If an environment variable is non-existing, the following line will panic.
                let arguments: String = shellexpand::full(&arguments).unwrap().to_string();
                println!("    command: {} {}", command, arguments);
                let arguments: Vec<&str> = arguments.split(' ').collect();
                // println!("    command: {}   arguments: {:?}", command, arguments);
                let output = std::process::Command::new(command)
                                                       .args(&arguments)
                                                       .output()
                                                       .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

                // println!("output: {:#?}", output);


                // Print stdout
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));

                if output.status.success() {
                    match on_success {
                        OnSuccess::Continue => {},
                        OnSuccess::Echo {message} => {
                            println!("{}", message);
                        },
                        OnSuccess::Warn {message} => {
                            println!("WARNING: {}", message);
                        },
                        OnSuccess::Abort {message} => {
                            println!("ABORT: {}", message);
                            break;
                        },
                    }
                } else {
                    match on_failure {
                        OnFailure::Continue => {},
                        OnFailure::Echo {message} => {
                            println!("{}", message);
                        },
                        OnFailure::Warn {message} => {
                            println!("WARNING: {}", message);
                        },
                        OnFailure::Abort {message} => {
                            println!("ABORT: {}", message);
                            break;
                        },
                    }
                }
            },
            Step::SetEnv {variable, value} => {
                std::env::set_var(variable, value);
            },
            Step::Shell {name, command, stdout, on_success, on_failure} => {
                let shell = "sh";
                println!("    command: {} -c '{}'", shell, command);
                let output = std::process::Command::new(shell)
                                                       .arg("-c")
                                                       .arg(command)
                                                       .output()
                                                       .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

               // Print stdout
                match stdout {
                    Pipe::Null => { },
                    Pipe::StdOut => {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    },
                    Pipe::StdErr => {
                        // FIXME: Outout to stderr, not stdout
                        unimplemented!();
                    },
                    Pipe::Variable {name} => {
                        std::env::set_var(name, String::from_utf8_lossy(&output.stdout).into_owned());
                    },
                    Pipe::File {filename} => {
                        // FIXME: Save to file
                        unimplemented!();
                    }
                };

               if output.status.success() {
                   match on_success {
                       OnSuccess::Continue => {},
                       OnSuccess::Echo {message} => {
                           println!("{}", message);
                       },
                       OnSuccess::Warn {message} => {
                           println!("WARNING: {}", message);
                       },
                       OnSuccess::Abort {message} => {
                           println!("ABORT: {}", message);
                           break;
                       },
                   }
               } else {
                   match on_failure {
                       OnFailure::Continue => {},
                       OnFailure::Echo {message} => {
                           println!("{}", message);
                       },
                       OnFailure::Warn {message} => {
                           println!("WARNING: {}", message);
                       },
                       OnFailure::Abort {message} => {
                           println!("ABORT: {}", message);
                           break;
                       },
                   }
               }
            },
        }
    }

}

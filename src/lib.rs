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
enum Steps {
    Echo,
    Run,
    Shell,
    SetEnv,
}


fn parse_toml(filename: &str) -> Vec<Steps> {

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
    let mut steps = Vec::<Steps>::with_capacity(10);


    steps
}

pub fn execute_steps(filename: &str) {

    let steps = parse_toml(filename);

    println!("Steps: {:#?}", steps);
}

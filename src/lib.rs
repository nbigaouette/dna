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

    println!("Reading toml file: {}", filename);

    unimplemented!();
}

pub fn execute_steps(filename: &str) {
    println!("Reading file {:?}", filename);

    let steps = parse_toml(filename);

    println!("Steps: {:?}", steps);
}

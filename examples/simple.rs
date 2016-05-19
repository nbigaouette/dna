extern crate dna;

fn main() {
    println!("Hello from simple.rs");
    dna::test();

    dna::execute_steps("simple.toml");
}

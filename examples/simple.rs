extern crate dna;

fn main() {
    println!("Hello from simple.rs");
    dna::test();

    dna::read_steps("simple.toml");
}

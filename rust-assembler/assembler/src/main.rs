mod assembler;
use assembler::assemble;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let result = assemble(&input);
    for byte in &result {
    println!("{:08b}", byte); // prints 8-bits 
}
  
}
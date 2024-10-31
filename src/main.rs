mod checker;
mod compiler;
mod generator;

use std::io::{stdout, Write};

use checker::code_is_valid;
use compiler::{compile_code, run_code};
use generator::CombinationGenerator;

fn main() {
    let mut length: usize = 21; // smallest valid C program     int main(){return 0;}

    loop {
        print!("\rLength: {length}");
        stdout().flush().unwrap();

        let generator: CombinationGenerator = CombinationGenerator::new(length);

        for code in generator {
            if code_is_valid(&code) && compile_code(&code) && run_code() {
                println!("Valid C code: {}", code);
            }
        }

        length += 1;
    }
}

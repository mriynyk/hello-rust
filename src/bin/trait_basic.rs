#![allow(unused)]

struct Solidity {
    version: String,
}

struct Viper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait Test {
    fn test(&self, file_path: &str) -> String {
        format!("test {file_path}")
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
}

impl Compiler for Viper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {file_path}")
    }
}

impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("forge {file_path}")
    }
}

impl Test for Viper {}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };

    let vy = Viper {
        version: "0.4".to_string(),
    };

    println!("compile sol: {}", compile(&sol, "Hello.sol"));
    println!("compile vy: {}", compile(&vy, "Hello.vy"));
    println!("test sol: {}", sol.test("Hello.sol"));
    println!("test vy: {}", vy.test("Hello.vy"));
}

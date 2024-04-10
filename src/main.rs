use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use dialoguer::Select;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ascii_art = r#"
██   ██ ███████ ██      ██       ██████      ███████ ██      ██    ██ ███████ ███    ██ ████████ 
██   ██ ██      ██      ██      ██    ██     ██      ██      ██    ██ ██      ████   ██    ██    
███████ █████   ██      ██      ██    ██     █████   ██      ██    ██ █████   ██ ██  ██    ██    
██   ██ ██      ██      ██      ██    ██     ██      ██      ██    ██ ██      ██  ██ ██    ██    
██   ██ ███████ ███████ ███████  ██████      ██      ███████  ██████  ███████ ██   ████    ██    
                                                                                                 
                                                                                                 
"#;

    println!("{}", ascii_art);
    println!("HelloFluent\n");

    let selections = ["Hardhat JavaScript (Solidity & Vyper)", "Hardhat TypeScript (Solidity & Vyper)", "Rust", "Exit"];
    let selection = Select::new()
        .with_prompt("Choose your setup")
        .default(0)
        .items(&selections[..])
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;  // Convert dialoguer error to io::Error

        match selection {
            0 => spin_js()?,
            1 => spin_ts()?,
            2 => spin_rust()?,
            3 => {
                println!("Exiting program.");
                return Ok(()); // Exit the program gracefully
            },
            _ => unreachable!(),
        };

    Ok(())
}
fn spin_js() -> io::Result<()>{

    const VYPER_SC: &str = include_str!("contract-templates/hello-v.vy");
    const SOL_SC: &str = include_str!("contract-templates/hello.sol");
    const SOL_SCRIPT: &str  = include_str!("js-template/deploy.js");
    const VYPER_SCRIPT : &str = include_str!("js-template/deployvyper.js");
    const HARDHAT_CONFIG : &str = include_str!("js-template/hardhat.config.js");
    const PACKAGE_JSON : &str = include_str!("js-template/package.json");

    create_file_with_content("contracts/hello-v.vy", VYPER_SC)?;
    create_file_with_content("contracts/hello.sol", SOL_SC)?;
    create_file_with_content("scripts/deploy.js", SOL_SCRIPT)?;
    create_file_with_content("scripts/deployvyper.js", VYPER_SCRIPT)?;
    create_file_with_content("hardhat.config.js", HARDHAT_CONFIG)?;
    create_file_with_content("package.json", PACKAGE_JSON)?;

    Ok(())
}
fn copy_file(source_path: &str, target_path: &str) -> io::Result<()> {
    // Read the source file
    let mut original_file = File::open(source_path)?;
    let mut contents = String::new();
    original_file.read_to_string(&mut contents)?;

    // Check if target_path has a parent directory and if it needs to be created
    if let Some(target_dir) = Path::new(target_path).parent() {
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)?;
        }
    }

    // Create and write to the new file at target_path
    let mut new_file = File::create(target_path)?;
    new_file.write_all(contents.as_bytes())?;

    Ok(())
}
fn create_file_with_content(output_path: &str, content: &str) -> io::Result<()> {
    // Check if the output path has a parent directory and create it if necessary
    if let Some(parent_dir) = Path::new(output_path).parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    // Create and write the content to the new file at the output path
    let mut file = File::create(output_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn spin_rust() -> io::Result<()>{

    println!("Creating Rust Project ..");
    const LIB: &str = include_str!("rust-template/lib.rs");
    const GREET_SC: &str = include_str!("rust-template/greeting.rs");
    const CARGO: &str = include_str!("rust-template/Cargo.toml");
    const MAKE_FILE : &str = include_str!("rust-template/Makefile");
    const STACK_S : &str = include_str!("rust-template/stack.s");
    const GIT_IG : &str = include_str!("rust-template/gitignore.txt");

    create_file_with_content("src/lib.rs", LIB)?;
    create_file_with_content("src/greeting.rs", GREET_SC)?;
    create_file_with_content("Cargo.toml", CARGO)?;
    create_file_with_content("Makefile", MAKE_FILE)?;
    create_file_with_content("stack.s", STACK_S)?;
    create_file_with_content(".gitignore", GIT_IG)?;

    println!("Rust template created sucessfully");
    
    Ok(())
}

fn spin_ts() -> io::Result<()>{
    println!("Creating Typescript Project ..");
    const VYPER_SC: &str = include_str!("contract-templates/hello-v.vy");
    const SOL_SC: &str = include_str!("contract-templates/hello.sol");
    const SOL_SCRIPT: &str  = include_str!("ts-template/deploy.ts");
    const VYPER_SCRIPT : &str = include_str!("ts-template/deployvyper.ts");
    const HARDHAT_CONFIG : &str = include_str!("ts-template/hardhat.config.ts");
    const PACKAGE_JSON : &str = include_str!("ts-template/package.json");
    const TS_CONFIG: &str = include_str!("ts-template/tsconfig.json");

    create_file_with_content("contracts/hello.sol", SOL_SC)?;
    create_file_with_content("contracts/hello-v.vy", VYPER_SC)?;
    create_file_with_content("scripts/deploy.ts", SOL_SCRIPT)?;
    create_file_with_content("scripts/deployvyper.ts", VYPER_SCRIPT)?;
    create_file_with_content("hardhat.config.ts", HARDHAT_CONFIG)?;
    create_file_with_content("package.json", PACKAGE_JSON)?;
    create_file_with_content("tsconfig.json", TS_CONFIG)?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use std::{fs::{self, File}, io::{self, Read, Write}};

    use crate::spin_js;


    #[test]
    fn read_and_write()-> io::Result<()>{
    let mut original_file = File::open("src/contract-templates/hello.sol")?;

    // Read the contents of the file
    let mut contents = String::new();
    original_file.read_to_string(&mut contents)?;

    // Create the new file and write the contents to it
    fs::create_dir_all("output")?;
    let mut new_file = File::create("output/output-hello.sol")?;
    new_file.write_all(contents.as_bytes())?;
    Ok(())
    }
    #[test]
    fn test_spin_js()-> io::Result<()>{
        Ok(spin_js()?)
    }
}
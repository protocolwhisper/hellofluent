use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use dialoguer::Select;
use anyhow::Result;
use anyhow::anyhow;

fn main() -> Result<(())> {
    let args: Vec<String> = env::args().collect();
    let use_erc20 = args.len() > 1 && args[1] == "--erc20";
    //let blended_app = args.len() > 1  && args[1] == "--blendedapp";

    // if blended_app {
    //     spin_blended_app()?;
    //     return Ok(());
    // }else{
    //     return Err(anyhow!("Not a valid command : try gblend --blendedapp (Rust & Solidity) "));
    // }

    let ascii_art = r#"
    ██████╗ ██████╗ ██╗     ███████╗███╗   ██╗██████╗ 
    ██╔════╝ ██╔══██╗██║     ██╔════╝████╗  ██║██╔══██╗
    ██║  ███╗██████╔╝██║     █████╗  ██╔██╗ ██║██║  ██║
    ██║   ██║██╔══██╗██║     ██╔══╝  ██║╚██╗██║██║  ██║
    ╚██████╔╝██████╔╝███████╗███████╗██║ ╚████║██████╔╝
     ╚═════╝ ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═══╝╚═════╝                                                                                                                                                                           
"#;

    println!("{}", ascii_art);
    println!("Welcome to gblend dev tool 🚀\n");

    let selections = ["Hardhat JavaScript (Solidity & Vyper)", "Hardhat TypeScript (Solidity & Vyper)", "Rust", "Exit"];
    let selection = Select::new()
        .with_prompt("Choose your setup")
        .default(0)
        .items(&selections[..])
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;  // Convert dialoguer error to io::Error

        match selection {
            0 => spin_js(use_erc20)?,
            1 => spin_ts(use_erc20)?,
            2 => spin_rust()?,
            3 => {
                println!("Exiting program.");
                return Ok(()); // Exit the program gracefully
            },
            _ => unreachable!(),
        };

    Ok(())
}
fn spin_js(use_erc20: bool) -> io::Result<()>{
    if use_erc20{
        //Contracts
        const ERC20: &str = include_str!("contract-templates/erc20per.vy");
        const ERC20SOL: &str =  include_str!("contract-templates/erc20.sol");
        //Deploy files
        const DEPLOY_ERC20: &str = include_str!("js-template/deployerc20.js");
        const DEPLOY_VYPER: &str = include_str!("js-template/deployvy20.js");
        create_file_with_content("contracts/erc20.sol", ERC20SOL)?;
        create_file_with_content("contracts/erc20per.vy", ERC20)?;
        create_file_with_content("scripts/deployerc20.js", DEPLOY_ERC20)?;
        create_file_with_content("scripts/deployvy20.js", DEPLOY_VYPER)?;
    }else{
        const VYPER_SC: &str = include_str!("contract-templates/hello-v.vy");
        const SOL_SC: &str = include_str!("contract-templates/hello.sol");
        const SOL_SCRIPT: &str  = include_str!("js-template/deploy.js");
        const VYPER_SCRIPT : &str = include_str!("js-template/deployvyper.js");
        create_file_with_content("scripts/deploy.js", SOL_SCRIPT)?;
        create_file_with_content("scripts/deployvyper.js", VYPER_SCRIPT)?;
        create_file_with_content("contracts/hello-v.vy", VYPER_SC)?;
        create_file_with_content("contracts/hello.sol", SOL_SC)?;
    }

    const HARDHAT_CONFIG : &str = include_str!("js-template/hardhat.config.js");
    const PACKAGE_JSON : &str = include_str!("js-template/package.json");
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

//Create dir 
fn create_directories(output_path: &str) -> io::Result<()> {
    // Check if the output path has a parent directory and create it if necessary
    if let Some(parent_dir) = Path::new(output_path).parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    // Create the directory at the output path
    fs::create_dir_all(output_path)?;

    Ok(())
}

fn spin_rust() -> io::Result<()>{

    println!("Creating Rust Project ..");
    const LIB: &str = include_str!("rust-template/lib.rs");
    const CARGO: &str = include_str!("rust-template/Cargo.txt");
    const MAKE_FILE : &str = include_str!("rust-template/Makefile");
    const GIT_IG : &str = include_str!("rust-template/gitignore.txt");
    const DEPLOYOR : &str = include_str!("deployer/deployer.js");
    const PACKAGE_J: &str =  include_str!("deployer/package.json");

    create_file_with_content("src/lib.rs", LIB)?;
    create_file_with_content("Cargo.toml", CARGO)?;
    create_file_with_content("Makefile", MAKE_FILE)?;
    create_file_with_content(".gitignore", GIT_IG)?;
    create_file_with_content("deployer/deployer.js", DEPLOYOR)?;
    create_file_with_content("deployer/package.json", PACKAGE_J)?;
    println!("Rust template created sucessfully");
    
    Ok(())
}

fn spin_ts(use_erc20: bool) -> io::Result<()>{
    println!("Creating Typescript Project ..");
    if use_erc20{
        const ERC20: &str = include_str!("contract-templates/erc20per.vy");
        const ERC20SOL: &str =  include_str!("contract-templates/erc20.sol");
        //Deploy files
        const DEPLOY_ERC20: &str = include_str!("ts-template/deployerc20.ts");
        const DEPLOY_VYPER: &str = include_str!("ts-template/deploy20vyper.ts");
        create_file_with_content("contracts/erc20.sol", ERC20SOL)?;
        create_file_with_content("contracts/erc20per.vy", ERC20)?;
        create_file_with_content("scripts/deployerc20.ts", DEPLOY_ERC20)?;
        create_file_with_content("scripts/deployvy20.ts", DEPLOY_VYPER)?;
    }else {
        const VYPER_SC: &str = include_str!("contract-templates/hello-v.vy");
        const SOL_SC: &str = include_str!("contract-templates/hello.sol");
        const SOL_SCRIPT: &str  = include_str!("ts-template/deploy.ts");
        const VYPER_SCRIPT : &str = include_str!("ts-template/deployvyper.ts");
        create_file_with_content("contracts/hello.sol", SOL_SC)?;
        create_file_with_content("contracts/hello-v.vy", VYPER_SC)?;
        create_file_with_content("scripts/deploy.ts", SOL_SCRIPT)?;
        create_file_with_content("scripts/deployvyper.ts", VYPER_SCRIPT)?;
    }
    // Base file for typescript project
    
    const HARDHAT_CONFIG : &str = include_str!("ts-template/hardhat.config.ts");
    const PACKAGE_JSON : &str = include_str!("ts-template/package.json");
    const TS_CONFIG: &str = include_str!("ts-template/tsconfig.json");
    create_file_with_content("hardhat.config.ts", HARDHAT_CONFIG)?;
    create_file_with_content("package.json", PACKAGE_JSON)?;
    create_file_with_content("tsconfig.json", TS_CONFIG)?;
    Ok(())
}

fn spin_blended_app() -> io::Result<()>{
    println!("Creating blended app ..");
    const HARDHAT_CONFIG : &str = include_str!("blendedapp/hardhatconfig.ts");
    const PACKAGE_JSON: &str = include_str!("blendedapp/package.json");
    const TS_CONFIG : &str = include_str!("blendedapp/tsconfig.json");
    const DEPLOYMENT_SCRIPT: &str =  include_str!("blendedapp/deploymentscript.ts");
    const GREETING_TASK: &str = include_str!("blendedapp/greetingtask.ts");
    const MAKE_FILE : &str = include_str!("blendedapp/Makefile.txt");
    const CARGO_TOML: &str =  include_str!("blendedapp/cargo.txt");

    create_directories("contracts")?;

    create_file_with_content("hardhat.config.ts", HARDHAT_CONFIG)?;
    create_file_with_content("package.json", PACKAGE_JSON)?;
    create_file_with_content("tsconfig.json", TS_CONFIG)?;
    create_file_with_content("tasks/random.ts", GREETING_TASK)?;
    create_file_with_content("deploy/00_deploy_contracts.ts", DEPLOYMENT_SCRIPT)?;
    create_file_with_content("rustapp/Makefile", MAKE_FILE)?;
    create_file_with_content("rustapp/Cargo.toml", CARGO_TOML)?;


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
        Ok(spin_js(true)?)
    }
}
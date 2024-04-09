use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use dialoguer::Select;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let solidity_content = r#"
    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;
    contract Hello {
        function main() public pure returns (string memory) {
            return "Hello, Solidity!";
        }
    }
    "#;

    let vyper_content = r#"
    # @version ^0.3.0
    # Create a string variable that can store maximum 100 characters
    greet: public(String[100])

    @external
    def __init__():
        self.greet = "Hello World"
    "#;

    let ascii_art = r#"
██   ██ ███████ ██      ██       ██████      ███████ ██      ██    ██ ███████ ███    ██ ████████ 
██   ██ ██      ██      ██      ██    ██     ██      ██      ██    ██ ██      ████   ██    ██    
███████ █████   ██      ██      ██    ██     █████   ██      ██    ██ █████   ██ ██  ██    ██    
██   ██ ██      ██      ██      ██    ██     ██      ██      ██    ██ ██      ██  ██ ██    ██    
██   ██ ███████ ███████ ███████  ██████      ██      ███████  ██████  ███████ ██   ████    ██    
                                                                                                 
                                                                                                 
"#;

    println!("{}", ascii_art);
    println!("HelloFluent\n");

    let selections = ["Hardhat JavaScript", "Hardhat TypeScript", "Rust"];
    let selection = Select::new()
        .with_prompt("Choose your setup")
        .default(0)
        .items(&selections[..])
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;  // Convert dialoguer error to io::Error

    match selection {
        0 => setup_hardhat_js(solidity_content, vyper_content),
        1 => setup_ts_hardhat(solidity_content, vyper_content),
        2 => setup_rust_code(),
        _ => unreachable!(),
    };

    Ok(())
}
fn spin_js() -> io::Result<()>{
    println!("Copying hello.sol to current directory...");
    copy_file("src/contract-templates/hello.sol", "hello.sol")?;

    println!("Copying hello-vy.vy to contracts directory...");
    copy_file("src/contract-templates/hello-v.vy", "contracts/hello-v.vy")?;

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

fn setup_hardhat_js(soliditysc: &str , vypersc: &str) -> io::Result<()> {
    let contracts_directory = "contracts";
    let scripts_directory = "scripts";
    let solidity_file = "hello.sol";
    let vyper_file = "hello.vy";
    let hardhat_config_file = "hardhat-config.js";
    let deploy_script_file = "deploy.js";
    let deployvyper_script_file = "deployvyper.js";
    let package_json_file = "package.json";
    let hardhat_config_content = r#"require("@nomicfoundation/hardhat-toolbox");
    /**
     * @type import('hardhat/config').HardhatUserConfig
     */
    module.exports = {
      networks: {
        fluent_devnet1: {
          url: 'https://rpc.dev1.fluentlabs.xyz/', 
          chainId: 1337, 
          accounts : [
            `0x${"ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"}` ], // Replace with the private key of the deploying account
        },
      },
      solidity: {
        version: '0.8.19', 
      },
    };"#;

    let deploy_script_content = r#"async function main() {
        const [deployer] = await ethers.getSigners();
      
        console.log("Deploying contracts with the account:", deployer.address);
      
        const token = await ethers.deployContract("Token");
      
        console.log("Token address:", await token.getAddress());
    }
      
    main()
        .then(() => process.exit(0))
        .catch((error) => {
          console.error(error);
          process.exit(1);
        });"#;

    let deployvyper_script_content = r#"async function main() {
        const [deployer] = await ethers.getSigners();
        console.log("Deploying contracts with the account:", deployer.address);
        // Compile your Vyper contract and deploy it
        const TokenFactory = await ethers.getContractFactory("vypertoken");
        const token = await TokenFactory.deploy("Hello" , "Fluent");
        console.log("Token address:", await token.getAddress());
    }
    main()
        .then(() => process.exit(0))
        .catch((error) => {
            console.error(error);
            process.exit(1);
        });"#;

    let package_json_content = r#"{
        "name": "hardhat-project",
        "version": "1.0.0",
        "description": "A Hardhat project",
        "scripts": {
            "compile": "hardhat compile",
            "test": "hardhat test",
            "deploy": "node scripts/deploy.js",
            "deploy-vyper": "node scripts/deployvyper.js"
        },
        "dependencies": {
            "@nomicfoundation/hardhat-toolbox": "^1.0.0"
        }
    }"#;

    // Create directories
    fs::create_dir_all(contracts_directory)?;
    fs::create_dir_all(scripts_directory)?;

    // Create and write content to files
    let hardhat_config_path = Path::new(hardhat_config_file);
    let solidity_file_path = Path::new(contracts_directory).join(solidity_file);
    let vyper_file_path = Path::new(contracts_directory).join(vyper_file);
    let deploy_script_path = Path::new(scripts_directory).join(deploy_script_file);
    let deployvyper_script_path = Path::new(scripts_directory).join(deployvyper_script_file);
    let package_json_path = Path::new(package_json_file);

    let mut hardhat_config_file = File::create(&hardhat_config_path)?;
    hardhat_config_file.write_all(hardhat_config_content.as_bytes())?;

    let mut solidity_file = File::create(&solidity_file_path)?;
    solidity_file.write_all(soliditysc.as_bytes())?;

    let mut vyper_file = File::create(&vyper_file_path)?;
    vyper_file.write_all(vypersc.as_bytes())?;

    let mut deploy_script_file = File::create(&deploy_script_path)?;
    deploy_script_file.write_all(deploy_script_content.as_bytes())?;

    let mut deployvyper_script_file = File::create(&deployvyper_script_path)?;
    deployvyper_script_file.write_all(deployvyper_script_content.as_bytes())?;

    let mut package_json_file = File::create(&package_json_path)?;
    package_json_file.write_all(package_json_content.as_bytes())?;

    println!("Required directories and files have been created successfully.");
    Ok(())
}

fn setup_ts_hardhat(soliditysc: &str, vypersc: &str) -> io::Result<()> {
    println!("Setting up Hardhat project with TypeScript...");

    let contracts_directory = "contracts";
    let scripts_directory = "scripts_ts"; // Assuming TypeScript scripts go in a different directory
    let solidity_file = "hello.sol";
    let vyper_file = "hello.vy";
    let hardhat_config_file = "hardhat.config.ts"; // TypeScript Hardhat config
    let deploy_script_file = "deploy.ts"; // TypeScript deploy script
    let deployvyper_script_file = "deployvyper.ts"; // TypeScript deploy Vyper script
    let package_json_file = "package.json";

    // TypeScript specific content can be added/modified here

    let package_json_content = r#"{
        "name": "hardhat-project",
        "version": "1.0.0",
        "description": "A Hardhat project",
        "scripts": {
            "compile": "hardhat compile",
            "test": "hardhat test",
            "deploy": "node scripts/deploy.js",
            "deploy-vyper": "node scripts/deployvyper.js"
        },
        "dependencies": {
            "@nomicfoundation/hardhat-toolbox": "^1.0.0"
        }
    }"#;
    let hardhat_config_content = r#"import { HardhatUserConfig } from "hardhat/types";

const config: HardhatUserConfig = {
    networks: {
        fluent_devnet1: {
            url: 'https://rpc.dev1.fluentlabs.xyz/',
            chainId: 1337,
            accounts: [`0x${"ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"}`],
        },
    },
    solidity: "0.8.19",
};

export default config;"#;

    let deploy_script_content = r#"import { ethers } from "ethers";

async function main() {
    const [deployer] = await ethers.getSigners();
    console.log("Deploying contracts with the account:", deployer.address);

    const token = await ethers.getContractFactory("Token");
    const deployedToken = await token.deploy();

    console.log("Token address:", deployedToken.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });"#;

    let deployvyper_script_content = r#"import { ethers } from "ethers";

async function main() {
    const [deployer] = await ethers.getSigners();
    console.log("Deploying contracts with the account:", deployer.address);

    const TokenFactory = await ethers.getContractFactory("vypertoken");
    const token = await TokenFactory.deploy("Hello", "Fluent");

    console.log("Token address:", token.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });"#;

    // Assume package.json might need updates for TypeScript project setup, or it could remain the same
    // For simplicity, I am not repeating package_json_content here; it would be the same or updated as needed

    // Create directories
    fs::create_dir_all(contracts_directory)?;
    fs::create_dir_all(scripts_directory)?;

    // Create and write content to files
    let hardhat_config_path = Path::new(hardhat_config_file);
    let solidity_file_path = Path::new(contracts_directory).join(solidity_file);
    let vyper_file_path = Path::new(contracts_directory).join(vyper_file);
    let deploy_script_path = Path::new(scripts_directory).join(deploy_script_file);
    let deployvyper_script_path = Path::new(scripts_directory).join(deployvyper_script_file);
    // Assuming package.json is at the root and shared between setups

    let package_json_path = Path::new(package_json_file);

    let mut hardhat_config_file = File::create(&hardhat_config_path)?;
    hardhat_config_file.write_all(hardhat_config_content.as_bytes())?;

    let mut solidity_file = File::create(&solidity_file_path)?;
    solidity_file.write_all(soliditysc.as_bytes())?;

    let mut vyper_file = File::create(&vyper_file_path)?;
    vyper_file.write_all(vypersc.as_bytes())?;

    let mut deploy_script_file = File::create(&deploy_script_path)?;
    deploy_script_file.write_all(deploy_script_content.as_bytes())?;

    let mut deployvyper_script_file = File::create(&deployvyper_script_path)?;
    deployvyper_script_file.write_all(deployvyper_script_content.as_bytes())?;

    let mut package_json_file : File = File::create(&package_json_path)?;
    package_json_file.write_all(package_json_content.as_bytes())?;

    // package_json_file handling could be the same or different based on TypeScript needs
    // Here, I'm assuming it might get reused or updated as needed for the TypeScript environment
    println!("Required directories and files for TypeScript Hardhat project have been created successfully.");

    Ok(())
}

fn setup_rust_code() -> io::Result<()>{




    let rust_sc = r#"
    #![no_std]
    extern crate alloc;
    use alloc::string::String;

    extern crate fluentbase_sdk;

    use fluentbase_sdk::{SysPlatformSDK, SDK};

    #[no_mangle]
    pub extern "C" fn deploy() {
        // Deployment logic if any
    }

    #[no_mangle]
    pub extern "C" fn main() {
        let data = "Hello, World";
        SDK::sys_write(data.as_bytes());
    }
    "#;
    
    let cargo_toml_content = r#"[dependencies]
    fluentbase-sdk = { git = "https://github.com/fluentlabs-xyz/fluentbase", default-features = false, features = ["evm"] }

    [lib]
    crate-type = ["cdylib"]

    [profile.release]
    panic = "abort"
    lto = true
    opt-level = 'z'
    strip = true
    "#;
    
    let js_code = r#"
    const {Web3, ETH_DATA_FORMAT} = require('web3');
    const fs = require('fs');
    
    const DEPLOYER_PRIVATE_KEY = 'add your private key here';
    
    const main = async () => {
        if (process.argv.length < 3) {
            console.log(`You must specify path to the WASM binary!`);
            console.log(`Example: node deploy-contract.js --dev GIVE_PATH_HERE`);
            process.exit(-1);
        }
        let args = process.argv.slice(2);
        const checkFlag = (param) => {
            let indexOf = args.indexOf(param)
            if (indexOf < 0) {
                return false
            }
            args.splice(indexOf, 1)
            return true
        };
        let isLocal = checkFlag('--local')
        let isDev = checkFlag('--dev')
    
        let web3Url = 'https://rpc.dev0.fluentlabs.xyz/';
        if (isLocal) {
            web3Url = 'http://127.0.0.1:8545';
        }
    
        let [binaryPath] = args;
        let wasmBinary = fs.readFileSync(binaryPath).toString('hex');
        const web3 = new Web3(web3Url);
        let privateKey = process.env.DEPLOYER_PRIVATE_KEY || DEPLOYER_PRIVATE_KEY;
        let account = web3.eth.accounts.privateKeyToAccount('0x' + privateKey);
    
        console.log('Signing transaction...');
        const gasPrice = await web3.eth.getGasPrice(ETH_DATA_FORMAT)
        const signedTransaction = await web3.eth.accounts.signTransaction({
            data: '0x' + wasmBinary,
            gasPrice,
            gas: 1_000_000,
            from: account.address,
        }, privateKey)
    
        let contractAddress = '';
        console.log('Sending transaction...');
        await web3.eth.sendSignedTransaction(signedTransaction.rawTransaction)
           .on('confirmation', confirmation => {
               contractAddress = confirmation.receipt.contractAddress;
               console.log(confirmation)
               if (contractAddress) {
                   console.log(`Contract address is: ${contractAddress}`);
               }
           });
    
        const result = await web3.eth.call({
            to: contractAddress,
        });
        function isASCII(str) {
            return /^[\x00-\x7F]*$/.test(str);
        }
        if (isASCII(web3.utils.hexToAscii(result))) {
            console.log(`Message: "${web3.utils.hexToAscii(result)}"`)
        } else {
            console.log(`Message: "${result}"`)
        }
    
        // const signedTransaction1 = await web3.eth.accounts.signTransaction({
        //     to: contractAddress,
        //     gas: 1_000_000,
        // }, DEPLOYER_PRIVATE_KEY)
        // const receipt1 = await web3.eth.sendSignedTransaction(signedTransaction1.rawTransaction);
        // console.log(`Receipt: ${JSON.stringify(receipt1, null, 2)}`)
    
        const latestMinedBlockNumber = await web3.eth.getBlockNumber();
        console.log(`Latest block number: ${latestMinedBlockNumber}`);
    
        process.exit(0);
    };
    
    main().then(console.log).catch(console.error);
    "#;
    

    // Directory and file paths
    let src_directory = Path::new("src");
    let lib_rs_path = src_directory.join("lib.rs");
    let cargo_toml_path = Path::new("Cargo.toml");
    let deploy_contract_js_path = Path::new("deploy-contract.js");

    // Ensure source directory exists
    fs::create_dir_all(src_directory)?;

    // Write Rust smart contract to lib.rs
    let mut lib_rs_file = File::create(lib_rs_path)?;
    lib_rs_file.write_all(rust_sc.as_bytes())?;

    // Write Cargo.toml
    let mut cargo_toml_file = File::create(cargo_toml_path)?;
    cargo_toml_file.write_all(cargo_toml_content.as_bytes())?;

    // Write JavaScript deploy script
    let mut deploy_contract_js_file = File::create(deploy_contract_js_path)?;
    deploy_contract_js_file.write_all(js_code.as_bytes())?;

    println!("Files created successfully.");
    
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
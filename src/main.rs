use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the directory and file names
    let contracts_directory = "contracts";
    let scripts_directory = "scripts";
    let solidity_file = "hello.sol";
    let vyper_file = "hello.vy";
    let hardhat_config_file = "hardhat-config.js";
    let deploy_script_file = "deploy.js";
    let deployvyper_script_file = "deployvyper.js";
    let package_json_file = "package.json";

    // Content for each file
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
    @public
    def main() -> string {
        return "Hello, Vyper!";
    }
    "#;

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
    solidity_file.write_all(solidity_content.as_bytes())?;

    let mut vyper_file = File::create(&vyper_file_path)?;
    vyper_file.write_all(vyper_content.as_bytes())?;

    let mut deploy_script_file = File::create(&deploy_script_path)?;
    deploy_script_file.write_all(deploy_script_content.as_bytes())?;

    let mut deployvyper_script_file = File::create(&deployvyper_script_path)?;
    deployvyper_script_file.write_all(deployvyper_script_content.as_bytes())?;

    let mut package_json_file = File::create(&package_json_path)?;
    package_json_file.write_all(package_json_content.as_bytes())?;

    println!("Required directories and files have been created successfully.");
    Ok(())
}

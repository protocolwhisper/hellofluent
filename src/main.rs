use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Assuming this Rust program is run from the project root directory
    // which contains the 'templates' directory.

    let base_path = Path::new("templates");
    let contracts_directory = base_path.join("contracts");
    let scripts_directory = base_path.join("js-template/scripts");

    // Define the file names and paths
    let solidity_file_path = contracts_directory.join("hello.sol");
    let vyper_file_path = contracts_directory.join("hellovyper.vy"); // Updated file name to match your tree
    let hardhat_config_path = base_path.join("js-template/hardhat-config.json"); // Correct path inside js-template
    let deploy_script_path = scripts_directory.join("deploysol.js");
    let deployvyper_script_path = scripts_directory.join("deployvyper.js");
    let package_json_path = base_path.join("js-template/package.json"); // Correct path inside js-template

    println!("Reading files as string");

    // Read content from files
    let solidity_content = fs::read_to_string(solidity_file_path)?;
    let vyper_content = fs::read_to_string(vyper_file_path)?;
    let hardhat_config_content = fs::read_to_string(hardhat_config_path)?;
    let deploy_script_content = fs::read_to_string(deploy_script_path)?;
    let deployvyper_script_content = fs::read_to_string(deployvyper_script_path)?;
    let package_json_content = fs::read_to_string(package_json_path)?;

    // Output the contents to verify they're read correctly (optional)
    println!("Solidity content:\n{}", solidity_content);
    println!("Vyper content:\n{}", vyper_content);
    println!("Hardhat config content:\n{}", hardhat_config_content);
    println!("Deploy script content:\n{}", deploy_script_content);
    println!("Deploy Vyper script content:\n{}", deployvyper_script_content);
    println!("Package.json content:\n{}", package_json_content);

    Ok(())
}

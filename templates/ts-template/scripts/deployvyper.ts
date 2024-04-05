import { ethers } from "ethers";

async function main() {
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    // Assuming ethers.getContractFactory and deploy are properly typed
    const TokenFactory = await ethers.getContractFactory("vypertoken");
    const token = await TokenFactory.deploy("Hello", "Fluent");

    console.log("Token address:", await token.getAddress());
}

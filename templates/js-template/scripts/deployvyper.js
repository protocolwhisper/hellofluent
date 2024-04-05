async function main() {
    const [deployer] = await ethers.getSigners();

    console.log("Deploying contracts with the account:", deployer.address);

    // Compile your Vyper contract and deploy it
    const TokenFactory = await ethers.getContractFactory("hellovyper");
    const token = await TokenFactory.deploy("Hello" , "Fluent");

    console.log("Token address:", await token.getAddress());
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
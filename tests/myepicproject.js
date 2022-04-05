// script to tells anchor what we want to run and fncs to use

// import and save anchor tools
const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

// main function
const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  // create account keypair for program
  // this is created everytime we run anchor test
  const baseAccount = anchor.web3.Keypair.generate();

  // call fn and pass in params
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey, // account address
      user: provider.wallet.publicKey, // wallet address
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count Before: ', account.totalGifs.toString())

  // Call add_gif!
  await program.rpc.addGif("insert_a_giphy_link_here", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // fetch account data again
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count After: ', account.totalGifs.toString())
  console.log('👀 GIF List: ', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();

// script to tells anchor what we want to run and fncs to use

// import and save anchor tools
const anchor = require('@project-serum/anchor');

// main function
const main = async() => {
  console.log("🚀 Starting test...")
  
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Myepicproject;
  const tx = await program.rpc.startStuffOff();

  console.log("📝 Your transaction signature", tx);
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

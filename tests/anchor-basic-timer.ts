import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorBasicTimer } from '../target/types/anchor_basic_timer';

describe('anchor-basic-timer', () => {

  // Configure the client to use the local cluster.
  let provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorBasicTimer as Program<AnchorBasicTimer>;

  // Create user keys
  const user1 = anchor.web3.Keypair.generate();

  // Declare users Timer PDA and bump
  let user1TimerAddress = null;
  let user1TimerBump = null;


  // simple delay
  const delay = ms => new Promise(res => setTimeout(res, ms));

  it('Test Set Up', async () => {
    // Airdrop sol to user1
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user1.publicKey, anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );

    [user1TimerAddress, user1TimerBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("timer"), user1.publicKey.toBuffer()], 
      program.programId);

    console.log("User1 PubKey: ", user1.publicKey.toString());
    console.log("User1 Timer Address: ", user1TimerAddress.toString());
  });

  it('Initialize Timer', async () => {
    // Initialize the timer
    await provider.connection.confirmTransaction(
      await program.rpc.initializeTimer(
        user1TimerBump ,{
          accounts: {
            timer: user1TimerAddress,
            payer: user1.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
          signers: [user1]
      })
    );

    let timer = await program.account.timer.fetch(user1TimerAddress);
    let initializedTimestamp = timer.initializedTimestamp;

    console.log("Initialized Timestamp: ", initializedTimestamp.toNumber());
  });

  it('Run timer and log result', async () => {
    // Start timer
    await provider.connection.confirmTransaction(
      await program.rpc.startTimer({
          accounts: {
            timer: user1TimerAddress,
          }
      })
    );

    // Wait 9000 ms 
    await delay(9000);

    // Stop timer
    await provider.connection.confirmTransaction(
      await program.rpc.stopTimer({
          accounts: {
            timer: user1TimerAddress,
          }
      })
    );

    let timer = await program.account.timer.fetch(user1TimerAddress);

    let elapsedTime = timer.stopTime.toNumber() - timer.startTime.toNumber();
    console.log("Elapsed Time: ", elapsedTime);

  });

});

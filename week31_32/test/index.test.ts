import { expect, test } from "bun:test";
import { Connection, Keypair, PublicKey, LAMPORTS_PER_SOL, SystemProgram, Transaction } from '@solana/web3.js';



const adminAccount = Keypair.generate();
const dataAccount = Keypair.generate();

const PROGRAM_ID = new PublicKey("0x7y7frkggxtri76b78yrrew5f67hinhy7o78u8poijnjbvtxzw343tfhjn876");
const connection = new Connection(" http://127.0.0.1:8899");
const COUNTER_SIZE = 20;

test("Account initialize", async () => {
    const txn = await connection.requestAirdrop(adminAccount.publicKey, 1 * LAMPORTS_PER_SOL)
    await connection.confirmTransaction(txn);

    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const createAccount = SystemProgram.createAccount({
        fromPubkey: adminAccount.publicKey,
        lamports,
        space: COUNTER_SIZE,
        programId: PROGRAM_ID,
        newAccountPubkey: dataAccount.publicKey
    });

    const createAccTxn = new Transaction();
    createAccTxn.add(createAccount);

    const signature = await connection.sendTransaction(createAccTxn, [adminAccount, dataAccount]);
    await connection.confirmTransaction(signature);
    console.log(dataAccount.publicKey.toBase58());

    const dataAccountInfo = await connection.getAccountInfo(dataAccount.publicKey);
    console.log(dataAccountInfo?.data);

    const counter = borsh.deserialize(schema, dataAccountInfo?.data);
    console.log(counter.count);
    expect(counter.count).toBe(0);
})

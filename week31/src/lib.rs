use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(counter_contract);

#[derive(BorshDeserialize, BorshSerialize)]
enum Instructiontype {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = Instructiontype::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        Instructiontype::Increment(val) => {
            msg!("executing increment");
            counter_data.count += val;
        }
        Instructiontype::Decrement(val) => {
            msg!("executing increment");
            counter_data.count -= val;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut());
    msg!("counter increade to: {}", counter_data.count);

    Ok(())
}

// use pattern matching for getting the error or use ? which will return the error if exe stops
// match account {
//     Ok(account_data) => msg!("account info: "),
//     Err(e) => Err(e),
// }
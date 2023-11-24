use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

// Enum describing possible entry point instructions
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum WorkshopProgramInstruction {
    Add {
        // value to add
        increment: u64,
    },
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WorkshopAccountData {
    pub value: u64,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Process instruction");

    let instruction = WorkshopProgramInstruction::try_from_slice(instruction_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    match instruction {
        WorkshopProgramInstruction::Add { increment } => add_calc(account, increment),
    }
}

fn add_calc(account: &AccountInfo, increment: u64) -> ProgramResult {
    msg!("Add");

    let mut account_data = WorkshopAccountData::try_from_slice(&account.data.borrow())?;
    account_data.value += increment;
    account_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Data = {}", account_data.value);
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u64>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        assert_eq!(
            WorkshopAccountData::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .value,
            0
        );

        let instruction = WorkshopProgramInstruction::Add { increment: 128 };
        let instruction_data = instruction.try_to_vec().unwrap();
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            WorkshopAccountData::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .value,
            128
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            WorkshopAccountData::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .value,
            256
        );
    }
}

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

/// Define the type of state stored in accounts
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Process instruction");
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

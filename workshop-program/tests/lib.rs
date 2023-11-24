use {
    borsh::BorshDeserialize,
    program_workshop::{process_instruction, WorkshopAccountData, WorkshopProgramInstruction},
    solana_program_test::*,
    solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Signer,
        transaction::Transaction,
    },
    std::mem,
};

#[tokio::test]
async fn test_workshop_program() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "program_workshop",
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        account_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u64>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let account = banks_client
        .get_account(account_pubkey)
        .await
        .expect("get_account")
        .expect("account not found");
    assert_eq!(
        WorkshopAccountData::try_from_slice(&account.data)
            .unwrap()
            .value,
        0
    );

    // Add once
    let instruction_data = WorkshopProgramInstruction::Add { increment: 123 };
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &instruction_data,
            vec![AccountMeta::new(account_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify changes
    let greeted_account = banks_client
        .get_account(account_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        WorkshopAccountData::try_from_slice(&greeted_account.data)
            .unwrap()
            .value,
        123
    );
}

use {
    solana_rpc_client::rpc_client::RpcClient,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signature::Signer,
        system_instruction, system_program, transaction::Transaction,
    },
    std::{thread::sleep, time::Duration},
};

pub fn request_airdrop(rpc_client: &RpcClient, beneficiary: &Pubkey, balance_sol: u64) {
    let sign = rpc_client.request_airdrop(beneficiary, LAMPORTS_PER_SOL * balance_sol);
    let Ok(sign) = sign else {
        panic!("failed to get airdrop");
    };

    loop {
        sleep(Duration::from_secs(1));
        if rpc_client.confirm_transaction(&sign).unwrap() {
            break;
        }
    }
}

fn main() {
    let rpc_client = RpcClient::new("http://localhost:8899");

    // 1. Create authority and airdrop it some funds
    let payer = Keypair::new();
    request_airdrop(&rpc_client, &payer.pubkey(), 11);
    let balance = rpc_client.get_balance(&payer.pubkey()).unwrap();
    println!("Balance of {} is {} lamports", payer.pubkey(), balance);
    assert!(balance != 0, "Payer must have some not 0 balance");

    // 2. Create account:
    // * create keypair for the new account
    // * create transaction to create account (first try with random number for lammports to show what is rent exempt)
    // * send this transaction
    let account_keypair = Keypair::new();
    let space = 1024;
    let rent = rpc_client
        .get_minimum_balance_for_rent_exemption(space)
        .expect("must get rent exempt");
    println!("Rent exempt for account of {space}b: {rent}");
    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &account_keypair.pubkey(),
        rent,
        space as u64,
        &system_program::ID,
    );
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("client must give us blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[create_account_instruction],
        Some(&payer.pubkey()),
        &[&payer, &account_keypair],
        blockhash,
    );

    let _sign = rpc_client
        .send_and_confirm_transaction(&tx)
        .expect("transaction must be delivered");

    let acc = rpc_client
        .get_account(&account_keypair.pubkey())
        .expect("get_account works");
    println!("Here it is my new account: {:?}", acc);

    // 3. Write on-chain program
    // To deploy it `solana -ul program deploy target/deploy/program_workshop.so  --program-id target/deploy/program_workshop-keypair.json``

    // 4. Create instruction for our program
}

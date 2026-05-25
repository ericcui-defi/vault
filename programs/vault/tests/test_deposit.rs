use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas, prelude::Pubkey},
    litesvm::LiteSVM,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_deposit() {

    // Obtaining/creating operation metadata
    let program_id = vault::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // Deriving vault pda
    let (vault_pda, _bump) = Pubkey::find_program_address(
        &[b"vault", payer.pubkey().as_ref()],
        &program_id
    );

    // Creating "create vault" instruction
    let vault_creation_instruction = Instruction::new_with_bytes(
        program_id,
        &vault::instruction::Initialize {}.data(),
        vault::accounts::Initialize {
            user: payer.pubkey(),
            my_account: vault_pda,
            system_program: anchor_lang::solana_program::system_program::id()
        }.to_account_metas(None),
    );

    // Submitting vault-creation instruction
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[vault_creation_instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let balance_before = svm.get_balance(&vault_pda).unwrap();

    // Creating "deposit" instruction
    let vault_deposit_instruction = Instruction::new_with_bytes(
        program_id,
        &vault::instruction::Deposit {amount: 500_000_000}.data(),
        vault::accounts::Deposit{
            user: payer.pubkey(),
            vault: vault_pda,
            system_program: anchor_lang::solana_program::system_program::id()
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[vault_deposit_instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    // Asserting successful transaction
    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    // Asserting expected balance (plus rent)
    let vault_balance = svm.get_balance(&vault_pda).unwrap();
    assert_eq!(vault_balance, 500_000_000 + balance_before);

}
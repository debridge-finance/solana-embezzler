use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, log::sol_log_compute_units,
    msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let points: usize = usize::from_be_bytes(instruction_data.try_into().unwrap());
    let mut sum = 0f32;
    for point in 0..points {
        sum += ((point.leading_ones() + point.leading_zeros()) as f32).log2();
    }
    msg!("{}", sum);
    sol_log_compute_units();
    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        assert_matches::*,
        solana_program::instruction::{AccountMeta, Instruction},
        solana_program_test::*,
        solana_sdk::{signature::Signer, transaction::Transaction},
    };

    #[tokio::test]
    async fn test_transaction() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: 75usize.to_be_bytes().to_vec(),
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
}

use solana_program::{declare_id, msg};
use solana_program::program_error::ProgramError;
use {
    crate::{error::TokenError, processor::Processor},
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
};
use firewall::AccountPlan;
use firewall::Constraints;

declare_id!("extw959P4WToez4DkuXwNsJszqGpe3HuY56AcG5yevx");
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let mut plan = AccountPlan::new(&mut iter, 3);
    match instruction_data[0] {
        0 => {
            simple_ix(&mut plan, instruction_data)
        },
        1 => {
            optional_account_ix(&mut plan, instruction_data)
        },
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn pda_of_this_program_constraint(account_type: String) -> Constraints {
    let seeds = &[
        b"some_prefix".as_ref(),
        &account_type.as_bytes(),
    ];
    Constraints::pda(seeds.as_ref(), crate::id(), true, true)
}

fn simple_ix(plan: &mut AccountPlan, ix_data: &[u8]) -> ProgramResult {
    let payer = plan.required_account("payer", Constraints::payer())?;
    let mut subject = plan.required_account(
        "subject",
        pda_of_this_program_constraint("subject".to_string()),
    )?;
    plan.system_program()?;

    subject.initialize_account(10, &payer)?;

    Ok(())
}

fn optional_account_ix(plan: &mut AccountPlan, ix_data: &[u8]) -> ProgramResult {
    let payer = plan.required_account("payer", Constraints::payer())?;
    plan.system_program()?;
    let subject = plan.optional_account(
        "subject",
        pda_of_this_program_constraint("subject".to_string()),
    )?;

    if let Some(mut subject) = subject {
        subject.initialize_account(10, &payer)?;
    } else {
        msg!("Nothing to do");
    }

    Ok(())
}
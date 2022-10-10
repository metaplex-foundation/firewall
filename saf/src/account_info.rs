use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use crate::{AccountsError, Constraints};

pub struct AccountInfoContext<'entry, 'action> {
    pub name: &'static str,
    pub info: AccountInfo<'entry>,
    pub bump: Option<u8>,
    pub constraints: Constraints<'action>,
}


impl<'entry, 'action> AccountInfoContext<'entry, 'action> {
    pub fn mut_data(&'entry mut self) -> &'entry mut Rc<RefCell<&'entry mut [u8]>> {
        let info = self.info.borrow_mut();
        info.data.borrow_mut()
    }

    pub fn initialize_account(mut self,
                              initial_size: u64,
                              owner: &Pubkey,
                              payer: &AccountInfoContext<'entry, 'action>,
    ) -> Result<(), ProgramError> {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(initial_size as usize);
        invoke_signed(
            &system_instruction::create_account(payer.info.key, self.info.key, lamports, initial_size, owner),
            &[payer.info.clone(), self.info.clone()],
            &[&[self.constraints.seeds.unwrap(), &[&[self.bump.unwrap()]]].concat()],
        )?;
        Ok(())
    }
}
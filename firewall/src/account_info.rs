use std::cell::RefMut;
use solana_program::account_info::AccountInfo;
use solana_program::program::invoke_signed;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use crate::Constraints;

pub struct AccountInfoContext<'entry, 'action> {
    pub name: &'static str,
    pub info: &'entry AccountInfo<'entry>,
    pub bump: Option<u8>,
    pub constraints: Constraints<'action>,
}


impl<'entry, 'action> AccountInfoContext<'entry, 'action> {
    pub fn mut_data(&mut self) -> RefMut<'entry, &'entry mut [u8]> {
        self.info.data.borrow_mut()
    }

    pub fn initialize_account(&mut self,
                              initial_size: u64,
                              payer: &AccountInfoContext<'entry, 'action>,
    ) -> Result<(), DigitalAssetProtocolError> {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(initial_size as usize);
        invoke_signed(
            &system_instruction::create_account(payer.info.key, self.info.key, lamports, initial_size, &crate::id()),
            &[payer.info.clone(), self.info.clone()],
            &[&[self.constraints.seeds.unwrap(), &[&[self.bump.unwrap()]]].concat()],
        )?;
        Ok(())
    }
}
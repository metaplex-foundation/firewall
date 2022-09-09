use crate::account_constraints::AccountConstraints;
use crate::account_info::AccountInfoContext;
use crate::Constraints;

pub struct AccountPlan<'entry> {
    required_accounts: usize,
    accounts: &'entry [AccountInfo<'entry>],
}

impl<'entry> AccountWrapper<'entry> {
    pub fn new(accounts: &'entry [AccountInfo<'entry>]) -> Result<Self, DigitalAssetProtocolError> {
        Ok(AccountWrapper {
            accounts,
        })
    }

    pub fn system_program<'action>(&mut self, index: usize) -> Result<AccountInfoContext<'entry, 'action>, DigitalAssetProtocolError> {
        self.prepare_account(index, "system", Constraints::system_program())
    }

    pub fn prepare_account<'action>(&mut self, index: usize, name: &'static str, constraints: Constraints<'action>) -> Result<AccountInfoContext<'entry, 'action>, DigitalAssetProtocolError> {
        let mut accx = AccountInfoContext {
            name,
            info: &self.accounts[index],
            bump: None,
            constraints,
        };
        accx.validate_constraint()?;
        Ok(accx)
    }

    pub fn accounts_length(&self) -> usize {
        self.accounts.len()
    }
}

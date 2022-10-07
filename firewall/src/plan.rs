use std::iter::Peekable;
use solana_program::account_info::AccountInfo;
use crate::account_constraints::AccountConstraints;
use crate::account_info::AccountInfoContext;

use crate::{AccountsError, Constraints};

pub struct AccountPlan<'entry, 'action> {
    acct_iter: &'entry mut dyn Iterator<Item=AccountInfo<'entry>>,
    required_accounts: usize,
    curr: usize
}

impl<'entry, 'action> AccountPlan<'entry, 'action> {

    /// Create a new AccountPlan
    pub fn new(accounts_iter: &'entry mut dyn Iterator<Item=AccountInfo<'entry>>, required_size: usize) -> Self {
        AccountPlan {
            acct_iter: accounts_iter,
            required_accounts: required_size,
            curr: 0,
        }
    }

    /// Add a required account to the plan with the given constraints. This auto unwraps the account for convenience.
    pub fn required_account(&mut self, name: &'static str, constraints: Constraints<'action>) -> Result<AccountInfoContext, AccountsError> {
        self.prepare_account(name, constraints).and_then(|s| s.ok_or(AccountsError::RequiredAccountMissing))
    }

    /// Alias of [`prepare_account`](AccountPlan::prepare_account) for convenience.
    pub fn optional_account(&mut self, name: &'static str, constraints: Constraints<'action>) -> Result<Option<AccountInfoContext>, AccountsError> {
        self.prepare_account(name, constraints)
    }

    /// Add an account to the plan with the given constraints. This method consumes one item in the accounts iterator and wraps it with the context.
    /// Before returning the constraints are validated.
    pub fn prepare_account(&mut self, name: &'static str, constraints: Constraints<'action>) -> Result<
        Option<AccountInfoContext<'entry, 'action>>,
        AccountsError
    > {
        let fail = self.curr >= self.required_accounts;
        self.curr += 1;
        if let Some(a) = self.acct_iter.next() {
            let mut accx = AccountInfoContext {
                name,
                info: &a,
                bump: None,
                constraints,
            };
            accx.validate_constraint()?;
            return Ok(Some(accx));
        }
        if fail {
            Err(AccountsError::OutOfAccounts)
        } else {
            Ok(None)
        }
    }

    pub fn accounts_length(&self) -> usize {
        self.acct_iter.peekable().count()
    }
}

//TODO -> Macroize this
impl<'entry, 'action> AccountPlan<'entry, 'action> {
    /// Convenience method for adding a system program
    pub fn system_program<'action>(&mut self) -> Result<Option<AccountInfoContext<'entry, 'action>>, AccountsError> {
        self.prepare_account("system_program", Constraints::system_program())
    }

    /// Convenience method for adding a rent program
    pub fn rent<'action>(&mut self) -> Result<Option<AccountInfoContext<'entry, 'action>>, AccountsError> {
        self.prepare_account("rent", Constraints::rent())
    }

    /// Convenience method for adding lookup program
    pub fn address_lookup<'action>(&mut self) -> Result<Option<AccountInfoContext<'entry, 'action>>, AccountsError> {
        self.prepare_account("lookup", Constraints::address_lookup_program())
    }
}
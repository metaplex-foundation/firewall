use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::{Iter, IterMut};
use solana_program::account_info::AccountInfo;
use crate::account_constraints::AccountConstraints;
use crate::account_info::AccountInfoContext;

use crate::{AccountsError, Constraints};

// TODO IMPL Iter
pub struct AccountPlan<'entry> {
    accounts: &'entry [AccountInfo<'entry>],
    required_accounts: usize,
    curr: usize,
}

impl<'entry> AccountPlan<'entry> {
    /// Create a new AccountPlan
    pub fn new(accounts: &'entry [AccountInfo<'entry>], required_size: usize) -> Result<Self, AccountsError> {
        if accounts.len() < required_size {
            return Err(AccountsError::OutOfAccounts);
        }
        Ok(AccountPlan {
            accounts,
            required_accounts: required_size,
            curr: 0,
        })
    }

    /// Add a required account to the plan with the given constraints. This auto unwraps the account for convenience.
    pub fn required_account<'action>(&mut self, name: &'static str, constraints: Constraints<'action>) -> Result<AccountInfoContext<'entry, 'action>, AccountsError> {
        self.prepare_account(name, constraints)
            .and_then(|s|
                s.ok_or(AccountsError::RequiredAccountMissing)
            )
    }

    /// Alias of [`prepare_account`](AccountPlan::prepare_account) for convenience.
    pub fn optional_account<'action>(&mut self, name: &'static str, constraints: Constraints<'action>) -> Result<Option<AccountInfoContext<'entry, 'action>>, AccountsError> {
        self.prepare_account(name, constraints)
    }

    /// Add an account to the plan with the given constraints. This method consumes one item in the accounts iterator and wraps it with the context.
    /// Before returning the constraints are validated.
    pub fn prepare_account<'action>(&mut self, name: &'static str, constraints: Constraints<'action>) -> Result<
        Option<AccountInfoContext<'entry, 'action>>,
        AccountsError
    > {
        let fail = self.curr >= self.required_accounts;
        if let Some(a) = self.accounts.get(self.curr) {
            let mut accx = AccountInfoContext {
                name,
                info: a.clone(), // TODO -> There is a way to avoid this
                bump: None,
                constraints,
            };
            accx.validate_constraint()?;
            return Ok(Some(accx));
        }
        self.curr += 1;
        if fail {
            Err(AccountsError::OutOfAccounts)
        } else {
            Ok(None)
        }
    }

    pub fn accounts_length(&self) -> usize {
        self.accounts.len()
    }
}

//TODO -> Macroize this
impl<'entry> AccountPlan<'entry> {
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


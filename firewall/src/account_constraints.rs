use crate::account_info::AccountInfoContext;
use crate::utils::assert_key_equal;

pub trait AccountConstraints {
    fn validate_constraint(&mut self) -> Result<(), DigitalAssetProtocolError>;
}

impl<'entry, 'action> AccountConstraints for AccountInfoContext<'entry, 'action> {
    fn validate_constraint(&mut self) -> Result<(), DigitalAssetProtocolError> {
        if self.constraints.program && !self.info.executable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} needs to be a program", self.info.key)));
        }

        if !self.constraints.program && self.info.executable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} can't be a program", self.info.key)));
        }

        if self.constraints.writable && !self.info.is_writable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} needs to be writable", self.info.key)));
        }
        // May need to change this to support optional signers
        if self.constraints.signer && !self.info.is_signer {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} needs to be a signer", self.info.key)));
        }

        if let Some(ob) = self.constraints.owned_by {
            assert_key_equal(&ob, self.info.owner)?;
        }

        if self.constraints.empty && self.info.data_len() > 0 && self.info.lamports() > 0 {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} can't be a signer", self.info.key)));
        }

        if let Some(kef) = self.constraints.key_equals {
            assert_key_equal(&kef, self.info.key)?;
        }

        match (self.constraints.seeds, self.constraints.program_id) {
            (Some(seeds), Some(prg)) => {
                let (pubkey, bump) = derive(seeds, &prg);
                assert_key_equal(&pubkey, self.info.key)?;
                self.bump = Some(bump);
                Ok(())
            }
            (None, None) => Ok(()),
            _ => Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} has incorrect seeds", self.info.key)))
        }?;
        Ok(())
    }
}
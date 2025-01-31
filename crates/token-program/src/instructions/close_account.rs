use nostd_entrypoint_invoke::invoke_signed;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program_entrypoint::ProgramResult;

/// Close an account by transferring all its SOL to the destination account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
/// 
pub struct CloseAccount<'a> {
    /// Token Account.
    pub account: &'a NoStdAccountInfo,
    /// Destination Account
    pub destination: &'a NoStdAccountInfo,
    /// Owner Account
    pub authority: &'a NoStdAccountInfo,
}

impl CloseAccount<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.destination.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        let mut instruction_data = [0; 1];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 9;

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 1,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[self.account, self.destination, self.authority],
            signers,
        )
    }
}

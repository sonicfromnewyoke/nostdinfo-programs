use nostd_entrypoint_invoke::invoke_signed;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program_entrypoint::ProgramResult;

/// Given a native token account updates its amount field based
/// on the account's underlying `lamports`.
///
/// ### Accounts:
///   0. `[WRITE]`  The native token account to sync with its underlying
///      lamports.
pub struct SyncNative<'a> {
    /// Native Token Account
    pub native_token: &'a NoStdAccountInfo,
}

impl SyncNative<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 1] = [self.native_token.to_meta_c()];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        let mut instruction_data = [0; 1];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 17;

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 1,
            data: instruction_data.as_ptr(),
            data_len: 1,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.native_token], signers)
    }
}

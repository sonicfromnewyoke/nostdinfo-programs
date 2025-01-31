use nostd_entrypoint_invoke::invoke_signed;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program_entrypoint::ProgramResult;

/// Transfer Tokens from one Token Account to another.
///
/// ### Accounts:
///   0. `[WRITE]` Sender account
///   1. `[WRITE]` Recipient account
///   2. `[SIGNER]` Authority account
pub struct Transfer<'a> {
    /// Sender account.
    pub from: &'a NoStdAccountInfo,
    /// Recipient account.
    pub to: &'a NoStdAccountInfo,
    /// Authority account.
    pub authority: &'a NoStdAccountInfo,
    /// Amount of microtokens to transfer.
    pub amount: u64,
}

impl Transfer<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.from.to_meta_c(),
            self.to.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        // Instruction data layout:
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        let mut instruction_data = [0; 9];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 3;
        // Set amount as u64 at offset [1..9]
        instruction_data[1..9].copy_from_slice(&self.amount.to_le_bytes());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 9,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.from, self.to, self.authority], signers)
    }
}

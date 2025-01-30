use nostd_entrypoint_invoke::invoke_signed;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program_entrypoint::ProgramResult;
use solana_pubkey::Pubkey;

/// Initialize a new mint.
///
/// ### Accounts:
///   0. `[WRITABLE]` Mint account
///   1. `[]` Rent sysvar
pub struct InitializeMint<'a> {
    /// Mint Account.
    pub mint: &'a NoStdAccountInfo,
    /// Rent sysvar Account.
    pub rent_sysvar: &'a NoStdAccountInfo,
    /// Decimals.
    pub decimals: u8,
    /// Mint Authority.
    pub mint_authority: &'a Pubkey,
    /// Freeze Authority.
    pub freeze_authority: Option<&'a Pubkey>,
}

impl InitializeMint<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 2] =
            [self.mint.to_meta_c(), self.rent_sysvar.to_meta_c()];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1]: decimals (1 byte, u8)
        // -  [2..34]: mint_authority (32 bytes, Pubkey)
        // -  [34]: freeze_authority presence flag (1 byte, u8)
        // -  [35..67]: freeze_authority (optional, 32 bytes, Pubkey)
        let mut instruction_data = [0; 67];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 0;
        // Set decimals as u8 at offset [1]
        instruction_data[1] = self.decimals;
        // Set mint_authority as Pubkey at offset [2..34]
        instruction_data[2..34].copy_from_slice(self.mint_authority.as_ref());
        // Set COption & freeze_authority at offset [34..67]
        if let Some(freeze_authority) = self.freeze_authority {
            instruction_data[34] = 1;
            instruction_data[35..67].copy_from_slice(freeze_authority.as_ref());
        } else {
            instruction_data[34] = 0;
        }

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 2,
            data: instruction_data.as_ptr(),
            data_len: 67,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.mint, self.rent_sysvar], signers)
    }
}

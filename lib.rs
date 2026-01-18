use anchor_lang::prelude::*;

declare_id!("IdLyfe1111111111111111111111111111111111");

#[program]
pub mod identity_lifecycle_vault {
    use super::*;

    // ------------------------------------------------
    // Initialize Identity Vault (Enrollment)
    // ------------------------------------------------
    pub fn initialize_identity(ctx: Context<InitializeIdentity>) -> Result<()> {
        let vault = &mut ctx.accounts.identity_vault;
        let clock = Clock::get()?;

        vault.owner = ctx.accounts.owner.key();
        vault.created_at = clock.unix_timestamp;
        vault.last_update = clock.unix_timestamp;
        vault.update_count = 0;
        vault.lifecycle_score = 0;

        Ok(())
    }

    // ------------------------------------------------
    // Record Biometric Update
    // ------------------------------------------------
    pub fn biometric_update(ctx: Context<UpdateIdentity>) -> Result<()> {
        let vault = &mut ctx.accounts.identity_vault;
        let clock = Clock::get()?;

        require_keys_eq!(vault.owner, ctx.accounts.owner.key(), ErrorCode::Unauthorized);

        vault.update_count += 1;
        vault.lifecycle_score += BIOMETRIC_WEIGHT;
        vault.last_update = clock.unix_timestamp;

        Ok(())
    }

    // ------------------------------------------------
    // Record Demographic Update
    // ------------------------------------------------
    pub fn demographic_update(ctx: Context<UpdateIdentity>) -> Result<()> {
        let vault = &mut ctx.accounts.identity_vault;
        let clock = Clock::get()?;

        require_keys_eq!(vault.owner, ctx.accounts.owner.key(), ErrorCode::Unauthorized);

        vault.update_count += 1;
        vault.lifecycle_score += DEMOGRAPHIC_WEIGHT;
        vault.last_update = clock.unix_timestamp;

        Ok(())
    }

    // ------------------------------------------------
    // Close Identity Vault (Optional Deactivation)
    // ------------------------------------------------
    pub fn close_identity(ctx: Context<CloseIdentity>) -> Result<()> {
        let vault = &ctx.accounts.identity_vault;

        require_keys_eq!(vault.owner, ctx.accounts.owner.key(), ErrorCode::Unauthorized);

        Ok(())
    }
}

// ==================================================
// Account Contexts
// ==================================================

#[derive(Accounts)]
pub struct InitializeIdentity<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + IdentityVault::SIZE,
        seeds = [b"identity", owner.key().as_ref()],
        bump
    )]
    pub identity_vault: Account<'info, IdentityVault>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    #[account(
        mut,
        seeds = [b"identity", owner.key().as_ref()],
        bump
    )]
    pub identity_vault: Account<'info, IdentityVault>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseIdentity<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"identity", owner.key().as_ref()],
        bump
    )]
    pub identity_vault: Account<'info, IdentityVault>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

// ==================================================
// Identity Vault State
// ==================================================

#[account]
pub struct IdentityVault {
    pub owner: Pubkey,
    pub created_at: i64,
    pub last_update: i64,
    pub update_count: u64,
    pub lifecycle_score: u64,
}

impl IdentityVault {
    pub const SIZE: usize = 
        32 + // owner
        8  + // created_at
        8  + // last_update
        8  + // update_count
        8;   // lifecycle_score
}

// ==================================================
// Constants
// ==================================================

const BIOMETRIC_WEIGHT: u64 = 10;
const DEMOGRAPHIC_WEIGHT: u64 = 5;

// ==================================================
// Custom Errors
// ==================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized operation")]
    Unauthorized,
}

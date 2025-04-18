use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022, TransferChecked};
use anchor_spl::token::TokenAccount;

// This program is a vesting contract for distributing tokens over time.
declare_id!("program_id");

const SECONDS_IN_MONTH: i64 = 30 * 24 * 60 * 60;

// The vesting schedule is defined by the allocation type, which determines the total amount,

#[program]
pub mod vesting {
    use super::*;

    // The cliff duration, and the vesting duration.
    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        allocation_type: u8,
        start_time: i64,
    ) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;

        let (total_amount, cliff, duration) = match allocation_type {
            1 => (4_000_000_000, 6 * SECONDS_IN_MONTH, 24 * SECONDS_IN_MONTH),
            2 => (10_000_000_000, 1 * SECONDS_IN_MONTH, 16 * SECONDS_IN_MONTH),
            3 => (6_000_000_000, 0, 13 * SECONDS_IN_MONTH),
            4 => (2_000_000_000, 0, 8 * SECONDS_IN_MONTH),
            5 => (4_000_000_000, 12 * SECONDS_IN_MONTH, 12 * SECONDS_IN_MONTH),
            6 => (20_000_000_000, 0, 36 * SECONDS_IN_MONTH),
            _ => return Err(error!(VestingError::InvalidAllocationType)),
        };

        vesting.beneficiary = *ctx.accounts.beneficiary.key;
        vesting.total_amount = total_amount;
        vesting.claimed_amount = 0;
        vesting.cliff_duration = cliff;
        vesting.vesting_duration = duration;
        vesting.start_time = start_time;
        vesting.allocation_type = allocation_type;

        Ok(())
    }

    // The beneficiary can claim their tokens after the cliff period and during the vesting period.
    pub fn claim_vested_tokens(ctx: Context<ClaimVestedTokens>) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= vesting.start_time, VestingError::VestingNotStarted);
        require_keys_eq!(ctx.accounts.beneficiary.key(), vesting.beneficiary, VestingError::Unauthorized);

        let time_since_start = now - vesting.start_time;
        if time_since_start < vesting.cliff_duration {
            return Err(VestingError::CliffNotReached.into());
        }

        let vested = if time_since_start >= vesting.vesting_duration {
            vesting.total_amount
        } else {
            vesting.total_amount * time_since_start as u64 / vesting.vesting_duration as u64
        };

        let claimable = vested.saturating_sub(vesting.claimed_amount);
        require!(claimable > 0, VestingError::NothingToClaim);

        let signer_seeds: &[&[u8]] = &[
            b"vault_auth",
            vesting.to_account_info().key.as_ref(),
            &[ctx.bumps.vault_auth],
        ];

        let signer: &[&[&[u8]]] = &[signer_seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.beneficiary_ata.to_account_info(),
                authority: ctx.accounts.vault_auth.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            signer,
        );

        token_2022::transfer_checked(cpi_ctx, claimable, 9)?;
        vesting.claimed_amount += claimable;
        Ok(())
    }
}

// The VestingSchedule struct defines the vesting schedule for a beneficiary.
#[account]
pub struct VestingSchedule {
    pub beneficiary: Pubkey,
    pub total_amount: u64,
    pub claimed_amount: u64,
    pub cliff_duration: i64,
    pub vesting_duration: i64,
    pub start_time: i64,
    pub allocation_type: u8,
}

// The InitializeVesting struct defines the accounts required to initialize a vesting schedule.
// It includes the vesting account, the payer, the beneficiary, and the system program.

#[derive(Accounts)]
pub struct InitializeVesting<'info> {
    #[account(init, payer = payer, space = 8 + 128)]
    pub vesting: Account<'info, VestingSchedule>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub beneficiary: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// The ClaimVestedTokens struct defines the accounts required to claim vested tokens.
// It includes the vesting account, the vault, the beneficiary's token account, the mint,
// the token program, the vault authority, and the beneficiary.
#[derive(Accounts)]
pub struct ClaimVestedTokens<'info> {
    #[account(mut, has_one = beneficiary)]
    pub vesting: Account<'info, VestingSchedule>,

    /// CHECK: Token vault owned by vault_auth PDA
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    /// CHECK: Beneficiary's token account
    #[account(mut)]
    pub beneficiary_ata: AccountInfo<'info>,

    /// CHECK: Token mint (Token-2022)
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,

    /// CHECK: PDA authority, verified via seeds
    #[account(
        seeds = [b"vault_auth", vesting.key().as_ref()],
        bump
    )]
    pub vault_auth: AccountInfo<'info>,

    #[account(mut)]
    pub beneficiary: Signer<'info>,
}

// The VestingError enum defines the possible errors that can occur in the vesting program.
// These errors include issues with the vesting schedule, such as not starting on time,
// not reaching the cliff period, having nothing to claim, and invalid allocation types.
#[error_code]
pub enum VestingError {
    #[msg("Vesting has not started yet.")]
    VestingNotStarted,
    #[msg("Cliff period has not been reached.")]
    CliffNotReached,
    #[msg("No tokens available for claiming.")]
    NothingToClaim,
    #[msg("Invalid allocation type.")]
    InvalidAllocationType,
    #[msg("Unauthorized claim attempt.")]
    Unauthorized,
    #[msg("Vault does not have expected authority.")]
    InvalidVaultAuth,
}

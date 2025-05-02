use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022, TransferChecked};
use anchor_spl::token::{TokenAccount, Mint};

declare_id!("program_id");

const BASE_RATE: f64 = 0.0001;
const MAX_ENERGY_FACTOR: f64 = 1.0;
const MAINTENANCE_WINDOW: i64 = 60 * 24 * 60 * 60; // 60 days in seconds

// This program allows users to stake tokens and claim rewards based on the amount staked and the time elapsed since the last claim.
// The rewards are calculated using a base rate and the amount staked.
// The program also includes a lockup period during which the staked tokens cannot be withdrawn.
// The program uses a PDA (Program Derived Address) to manage the vault where the staked tokens are held.
// The PDA is derived from the stake account and is used to authorize transfers of tokens from the vault to the user's account.
// The program includes error handling to ensure that users cannot claim rewards if there are none available.
// The program also includes a system program to handle the creation of new accounts and the transfer of tokens.
// The program uses the Anchor framework to simplify the development process and provide a more secure and efficient way to manage accounts and tokens.
// The program is designed to be efficient and secure, with a focus on minimizing the risk of errors and vulnerabilities.
#[program]
pub mod staking {
    use super::*;

    pub fn initialize_stake(
        ctx: Context<InitializeStake>,
        amount: u64,
        lockup_period: i64,
        start_time: i64,
    ) -> Result<()> {
        let stake = &mut ctx.accounts.stake;

        stake.owner = ctx.accounts.owner.key();   // Store the owner's public key
        stake.amount = amount; // Store the amount staked
        stake.start_time = start_time; // Store the start time of the stake
        stake.lockup_period = lockup_period; // Store the lockup period
        stake.last_claimed = start_time; // Initialize the last claimed time to the start time

        // Transfer tokens from the owner's account to the vault
        let signer_seeds = &[b"vault_auth", stake.to_account_info().key.as_ref(), &[ctx.bumps.vault_auth]]; 
        let signer = &[&signer_seeds[..]];

        //  Create a CPI context for the token transfer
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.owner_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            signer,
        );

        token_2022::transfer_checked(cpi_ctx, amount, 9)?;
        Ok(())
    }
    /// Claim rewards based on the elapsed time since the last claim
    /// and the amount staked.
    /// The rewards are calculated using a base rate and the amount staked.
    /// The rewards are transferred from the vault to the user's account.
    /// The function checks if there are rewards available to claim
    /// and updates the last claimed time to the current time.
    /// The function uses a PDA (Program Derived Address) to authorize the transfer of tokens from the vault.
    /// The PDA is derived from the stake account and is used to authorize transfers of tokens from the vault to the user's account.
    /// The function includes error handling to ensure that users cannot claim rewards if there are none available.
    /// The function also includes a system program to handle the transfer of tokens.
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;

        let stake = &mut ctx.accounts.stake;
        let elapsed = now - stake.last_claimed;
        require!(elapsed > 0, ErrorCode::NothingToClaim);

        let reward = (BASE_RATE * elapsed as f64 * stake.amount as f64) as u64;
        stake.last_claimed = now;

        let signer_seeds = &[b"vault_auth", stake.to_account_info().key.as_ref(), &[ctx.bumps.vault_auth]];
        let signer = &[&signer_seeds[..]];

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

        token_2022::transfer_checked(cpi_ctx, reward, 9)?;
        Ok(())
    }
}

/// Define the PDA authority for the vault
/// The PDA is derived from the stake account and is used to authorize transfers of tokens from the vault to the user's account.
/// The PDA is created using the `vault_auth` seed and the stake account's public key.
/// The PDA is used to authorize transfers of tokens from the vault to the user's account.
/// The PDA is created using the `vault_auth` seed and the stake account's public key.
/// /// The PDA is used to authorize transfers of tokens from the vault to the user's acco
#[derive(Accounts)]
pub struct InitializeStake<'info> {
    #[account(init, payer = payer, space = 8 + 128)]
    pub stake: Account<'info, StakeAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub owner_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [b"vault_auth", stake.key().as_ref()],
        bump
    )]
    /// CHECK: PDA authority for vault control
    pub vault_auth: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// Define the context for the claim rewards function
/// The context includes the stake account, the owner of the stake,
/// the vault account, the beneficiary's associated token account,
/// the mint account, the PDA authority for the vault, and the token program.
#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut, has_one = owner)]
    pub stake: Account<'info, StakeAccount>,

    pub owner: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub beneficiary_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [b"vault_auth", stake.key().as_ref()],
        bump
    )]
    /// CHECK: PDA authority for vault control
    pub vault_auth: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,
}

/// Define the stake account structure
#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub lockup_period: i64,
    pub last_claimed: i64,
    // Add telemetry info fields later
}

#[error_code]
pub enum ErrorCode {
    #[msg("No rewards available to claim yet.")]
    NothingToClaim,
}

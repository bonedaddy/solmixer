use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use anchor_lang::solana_program::{pubkey::Pubkey, rent::Rent};

pub mod assets;
pub mod errors;

#[program]
pub mod solmixer {
    use super::*;
    use errors::*;
    #[state]
    pub struct Management {
        pub authority: Pubkey,
    }
    impl Management {
        pub fn new(ctx: Context<Auth>) -> Result<Self> {
            Ok(Management{
                authority: *ctx.accounts.authority.key,
            })
        }
    }
    /// creates a new laundromat which provides fund mixing services for a single asset
    pub fn new_laundromat(ctx: Context<CreateLaundromat>, asset: u64) -> Result<()> {
        if !assets::Asset::is_valid_asset(asset) {
            return Err(ErrorCode::InvalidAsset.into());
        }
        ctx.accounts.laundromat.asset = asset;
        Ok(())
    }
    pub fn deposit_into_laundromat(ctx: Context<DepositIntoLaundromat>, amount: u64) -> Result<()> {
        if ctx.accounts.authority.lamports() < amount {
            return Err(ErrorCode::Insufficientfunds.into());
        }
        let deposit_q = &mut ctx.accounts.deposit_q.load_mut()?;
        if deposit_q.num_deposits + 1 > 25000 {
            return Err(ErrorCode::TooManyDeposits.into());
        }
        let idx = deposit_q.num_deposits;
        deposit_q.deposits[idx as usize] = Deposit{
            from: *ctx.accounts.authority.key,
            amount: amount,
        };
        deposit_q.num_deposits += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct CreateLaundromat<'info> {
    #[account(init)]
    pub laundromat: ProgramAccount<'info, Laundromat>,
    #[account(init)]
    pub deposit_q: Loader<'info, DepositQ>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct DepositIntoLaundromat<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub laundromat: ProgramAccount<'info, Laundromat>,
    #[account(mut)]
    pub deposit_q: Loader<'info, DepositQ>,
}

#[account]
pub struct Laundromat {
    // the total number of washed funds available for withdrawal
    pub total_washed_funds: u64,
    // the total number of unwashed funds
    pub total_unwashed_funds: u64,
    pub asset: u64,
}  

  
#[account(zero_copy)]
pub struct DepositQ {
    // the particular laundromat this queue applies too
    pub laundromat: Pubkey,
    pub num_deposits: u64,
    // queued deposits for washing
    pub deposits: [Deposit; 25000],
}

#[zero_copy]
pub struct Deposit {
    pub from: Pubkey,
    pub amount: u64,
}


// see https://github.com/project-serum/anchor/blob/master/examples/zero-copy/programs/zero-copy/src/lib.rs#L193
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RpcDeposit {
    pub from: Pubkey,
    pub amount: u64,
}

impl From<RpcDeposit> for Deposit {
    fn from(d: RpcDeposit) -> Deposit {
        Deposit{
            from: d.from,
            amount: d.amount,
        }
    }
}


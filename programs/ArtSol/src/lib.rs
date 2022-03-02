use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod art_sol {
    use super::*;
    pub fn create_auction(
        ctx: Context<CreateAuction>,
        token_address: Pubkey,
        min_bid_amount: i64,
    ) -> Result<()> {
        if min_bid_amount <= 0 {
            return Err(ErrorCode::MinBidAmountTooSmall.into());
        }

        let auction = &mut ctx.accounts.auction;
        let creator = &ctx.accounts.creator;

        auction.creator = *creator.key;
        auction.token_address = token_address;
        auction.min_bid_amount = min_bid_amount;
        Ok(())
    }

    pub fn place_bid(ctx: Context<PlaceBid>, bid_amount: i64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let bidder = &ctx.accounts.bidder;

        if bid_amount <= auction.highest_bid_amount {
            return Err(ErrorCode::BidAmountTooSmall.into());
        }

        Ok(())
    }

    pub fn finalize_auction(ctx: Context<FinalizeAuction>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(init, payer=creator, space=Auction::LEN)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(address=system_program::ID)]
    system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    pub auction: Account<'info, Auction>,
    pub bidder: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    pub auction: Account<'info, Auction>,
    pub creator: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum AuctionStatus {
    Live,
    Cancelled,
    Sold,
}

#[account]
pub struct Auction {
    creator: Pubkey,
    token_address: Pubkey,
    min_bid_amount: i64,
    highest_bid_amount: i64,
    highest_bidder: Pubkey,
    status: AuctionStatus,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BID_AMOUNT_LENGTH: usize = 8;
const STATUS_LENGTH: usize = 8;

impl Auction {
    const LEN: usize =
        DISCRIMINATOR_LENGTH + (3 * PUBLIC_KEY_LENGTH) + (2 * BID_AMOUNT_LENGTH) + STATUS_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Minimum bid amount should be greater than zero.")]
    MinBidAmountTooSmall,
    #[msg("Bid should be greater than the current highest bid.")]
    BidAmountTooSmall,
}

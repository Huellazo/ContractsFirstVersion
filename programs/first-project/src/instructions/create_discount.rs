use crate::{
    constants::*,
    error::ErrorCode,
    state::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateDiscount<'info> {
    #[account(mut)]
    pub business: Signer<'info>,
    #[account(
        seeds = [BUSINESS_SEED, business.key().as_ref()],
        bump,
        constraint = business_account.authority == business.key() @ ErrorCode::Unauthorized
    )]
    pub business_account: Account<'info, BusinessAccount>,
    #[account(
        init,
        payer = business,
        space = 8 + Discount::INIT_SPACE,
        seeds = [DISCOUNT_SEED, business.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub discount: Account<'info, Discount>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_discount(
    ctx: Context<CreateDiscount>,
    id: u64,
    code: String,
    discount_percent: u8,
    min_purchase: u64,
    available_units: u32,
    valid_until: i64,
) -> Result<()> {
    require!(code.len() <= MAX_CODE_LEN, ErrorCode::Overflow);
    require!(discount_percent <= 100, ErrorCode::Overflow);

    let discount = &mut ctx.accounts.discount;
    discount.id = id;
    discount.business = ctx.accounts.business.key();
    discount.code = code;
    discount.discount_percent = discount_percent;
    discount.min_purchase = min_purchase;
    discount.available_units = available_units;
    discount.unlimited = available_units == 0;
    discount.valid_until = valid_until;
    discount.is_active = true;
    discount.bump = ctx.bumps.discount;
    Ok(())
}
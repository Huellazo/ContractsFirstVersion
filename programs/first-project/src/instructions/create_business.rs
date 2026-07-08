use crate::{
    constants::*,
    error::ErrorCode,
    state::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateBusiness<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + BusinessAccount::INIT_SPACE,
        seeds = [BUSINESS_SEED, authority.key().as_ref()],
        bump
    )]
    pub business: Account<'info, BusinessAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_business(ctx: Context<CreateBusiness>, name: String) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::Overflow);
    let business = &mut ctx.accounts.business;
    business.authority = ctx.accounts.authority.key();
    business.name = name;
    business.rating_sum = 0;
    business.rating_count = 0;
    business.is_active = true;
    business.bump = ctx.bumps.business;
    Ok(())
}

#[derive(Accounts)]
pub struct SetBusinessActive<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [BUSINESS_SEED, authority.key().as_ref()],
        bump,
        constraint = business.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub business: Account<'info, BusinessAccount>,
}

pub fn handle_set_business_active(ctx: Context<SetBusinessActive>, active: bool) -> Result<()> {
    ctx.accounts.business.is_active = active;
    Ok(())
}
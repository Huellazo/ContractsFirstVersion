use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePassport<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Passport::INIT_SPACE,
        seeds = [PASSPORT_SEED, user.key().as_ref()],
        bump
    )]
    pub passport: Account<'info, Passport>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_passport(ctx: Context<InitializePassport>) -> Result<()> {
    let passport = &mut ctx.accounts.passport;
    passport.owner = ctx.accounts.user.key();
    passport.level = 0;
    passport.total_xp = 0;
    passport.stamps = 0;
    passport.last_spot = Pubkey::default();
    passport.bump = ctx.bumps.passport;
    msg!("Huellazo: passport created for {}", passport.owner);
    Ok(())
}
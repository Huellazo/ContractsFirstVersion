use crate::{constants::*, error::ErrorCode, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBackendAdmin<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,
}

pub fn handle_init_config(ctx: Context<InitConfig>, backend_admin: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.backend_admin = backend_admin;
    config.bump = ctx.bumps.config;
    Ok(())
}

pub fn handle_update_backend_admin(
    ctx: Context<UpdateBackendAdmin>,
    backend_admin: Pubkey,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.config.admin,
        ctx.accounts.admin.key(),
        ErrorCode::Unauthorized
    );
    ctx.accounts.config.backend_admin = backend_admin;
    Ok(())
}
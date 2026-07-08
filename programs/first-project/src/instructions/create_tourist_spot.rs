use crate::{
    constants::*,
    error::ErrorCode,
    state::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateTouristSpot<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = 8 + TouristSpot::INIT_SPACE,
        seeds = [TOURIST_SPOT_SEED, &id.to_le_bytes()],
        bump
    )]
    pub spot: Account<'info, TouristSpot>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_tourist_spot(
    ctx: Context<CreateTouristSpot>,
    id: u64,
    name: String,
    latitude: i64,
    longitude: i64,
    radius: u32,
    base_xp_reward: u64,
    achievement_uri: String,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.config.admin,
        ctx.accounts.admin.key(),
        ErrorCode::Unauthorized
    );
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::Overflow);
    require!(achievement_uri.len() <= MAX_URI_LEN, ErrorCode::Overflow);

    let spot = &mut ctx.accounts.spot;
    spot.id = id;
    spot.authority = ctx.accounts.admin.key();
    spot.name = name;
    spot.latitude = latitude;
    spot.longitude = longitude;
    spot.radius = radius;
    spot.base_xp_reward = base_xp_reward;
    spot.achievement_uri = achievement_uri;
    spot.total_visitors = 0;
    spot.is_active = true;
    Ok(())
}

#[derive(Accounts)]
pub struct SetSpotActive<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [TOURIST_SPOT_SEED, &spot.id.to_le_bytes()],
        bump
    )]
    pub spot: Account<'info, TouristSpot>,
}

pub fn handle_set_spot_active(ctx: Context<SetSpotActive>, active: bool) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.config.admin,
        ctx.accounts.admin.key(),
        ErrorCode::Unauthorized
    );
    ctx.accounts.spot.is_active = active;
    Ok(())
}
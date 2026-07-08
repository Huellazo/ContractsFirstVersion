use crate::{
    constants::*,
    error::ErrorCode,
    proof::verify_location_proof,
    state::*,
    utils::apply_xp,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VisitLandmark<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [TOURIST_SPOT_SEED, &spot.id.to_le_bytes()],
        bump
    )]
    pub spot: Account<'info, TouristSpot>,
    #[account(
        mut,
        seeds = [PASSPORT_SEED, user.key().as_ref()],
        bump
    )]
    pub passport: Account<'info, Passport>,
    #[account(
        init,
        payer = user,
        space = 8 + GeoAchievement::INIT_SPACE,
        seeds = [GEO_ACHIEVEMENT_SEED, user.key().as_ref(), spot.key().as_ref()],
        bump
    )]
    pub achievement: Account<'info, GeoAchievement>,
    pub system_program: Program<'info, System>,
}

pub fn handle_visit_landmark(
    ctx: Context<VisitLandmark>,
    timestamp: i64,
    location_proof: [u8; 64],
) -> Result<()> {
    let spot = &ctx.accounts.spot;
    require!(spot.is_active, ErrorCode::SpotNotActive);
    require!(
        spot.authority != Pubkey::default(),
        ErrorCode::SpotNotActive
    );

    let mut message = Vec::with_capacity(32 + 32 + 8 + 8 + 8);
    message.extend_from_slice(ctx.accounts.user.key().as_ref());
    message.extend_from_slice(ctx.accounts.spot.key().as_ref());
    message.extend_from_slice(&timestamp.to_le_bytes());
    message.extend_from_slice(&spot.latitude.to_le_bytes());
    message.extend_from_slice(&spot.longitude.to_le_bytes());

    verify_location_proof(&ctx.accounts.config.backend_admin, &message, &location_proof)?;

    let spot_id = spot.id;
    let xp = spot.base_xp_reward.max(SPOT_XP_DEFAULT);
    let lat = spot.latitude;
    let lon = spot.longitude;

    let achievement = &mut ctx.accounts.achievement;
    achievement.user = ctx.accounts.user.key();
    achievement.spot = ctx.accounts.spot.key();
    achievement.timestamp = timestamp;
    achievement.location_proof = location_proof;
    achievement.shared_on_social = false;
    achievement.bump = ctx.bumps.achievement;
    achievement.xp_earned = xp;

    let passport = &mut ctx.accounts.passport;
    require_keys_eq!(passport.owner, ctx.accounts.user.key(), ErrorCode::Unauthorized);
    apply_xp(passport, xp);
    passport.stamps = passport
        .stamps
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;
    passport.last_spot = ctx.accounts.spot.key();

    ctx.accounts.spot.total_visitors = ctx
        .accounts
        .spot
        .total_visitors
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    msg!(
        "Huellazo: visited spot {} (lat {} lon {}) earned {} xp",
        spot_id,
        lat,
        lon,
        xp
    );
    Ok(())
}
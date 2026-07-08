use crate::{
    constants::*,
    error::ErrorCode,
    state::*,
    utils::{apply_xp, share_bonus},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ShareAchievement<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [GEO_ACHIEVEMENT_SEED, user.key().as_ref(), achievement.spot.as_ref()],
        bump
    )]
    pub achievement: Account<'info, GeoAchievement>,
    #[account(
        mut,
        seeds = [PASSPORT_SEED, user.key().as_ref()],
        bump,
        constraint = passport.owner == user.key() @ ErrorCode::Unauthorized
    )]
    pub passport: Account<'info, Passport>,
}

pub fn handle_share_achievement(ctx: Context<ShareAchievement>) -> Result<()> {
    let achievement = &mut ctx.accounts.achievement;
    require_keys_eq!(
        achievement.user,
        ctx.accounts.user.key(),
        ErrorCode::NotAchievementOwner
    );
    require!(!achievement.shared_on_social, ErrorCode::AlreadyShared);

    let bonus = share_bonus(achievement.xp_earned);
    achievement.shared_on_social = true;

    let passport = &mut ctx.accounts.passport;
    apply_xp(passport, bonus);

    msg!("Huellazo: shared achievement, bonus xp {}", bonus);
    Ok(())
}
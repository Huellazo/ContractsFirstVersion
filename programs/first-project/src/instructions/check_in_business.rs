use crate::{
    constants::*,
    error::ErrorCode,
    state::*,
    utils::apply_xp,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    amount_spent: u64,
    _rating: u8,
    use_discount: bool,
    discount_id: Option<u64>,
    mission_id: Option<u64>,
    timestamp: i64
)]
pub struct CheckInBusiness<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: receives payment, validated via business_account.authority
    #[account(mut, constraint = business_wallet.key() == business_account.authority @ ErrorCode::Unauthorized)]
    pub business_wallet: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [BUSINESS_SEED, business_account.authority.as_ref()],
        bump
    )]
    pub business_account: Account<'info, BusinessAccount>,
    #[account(
        mut,
        seeds = [PASSPORT_SEED, user.key().as_ref()],
        bump,
        constraint = passport.owner == user.key() @ ErrorCode::Unauthorized
    )]
    pub passport: Account<'info, Passport>,
    #[account(
        init,
        payer = user,
        space = 8 + BusinessCheckIn::INIT_SPACE,
        seeds = [
            BUSINESS_CHECKIN_SEED,
            user.key().as_ref(),
            business_wallet.key().as_ref(),
            &timestamp.to_le_bytes()
        ],
        bump
    )]
    pub check_in: Account<'info, BusinessCheckIn>,
    #[account(
        mut,
        seeds = [
            DISCOUNT_SEED,
            business_wallet.key().as_ref(),
            &discount_id.unwrap_or(0).to_le_bytes()
        ],
        bump
    )]
    pub discount: Option<Account<'info, Discount>>,
    #[account(
        mut,
        seeds = [BUSINESS_MISSION_SEED, &mission_id.unwrap_or(0).to_le_bytes()],
        bump
    )]
    pub mission: Option<Account<'info, BusinessMission>>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserBusinessMissionProgress::INIT_SPACE,
        seeds = [
            BUSINESS_PROGRESS_SEED,
            user.key().as_ref(),
            &mission_id.unwrap_or(0).to_le_bytes()
        ],
        bump
    )]
    pub mission_progress: Option<Account<'info, UserBusinessMissionProgress>>,
    pub system_program: Program<'info, System>,
}

pub fn handle_check_in_business(
    ctx: Context<CheckInBusiness>,
    amount_spent: u64,
    rating: u8,
    use_discount: bool,
    _discount_id: Option<u64>,
    mission_id: Option<u64>,
    timestamp: i64,
) -> Result<()> {
    require!(
        rating >= RATING_MIN && rating <= RATING_MAX,
        ErrorCode::InvalidRating
    );
    require!(
        ctx.accounts.business_account.is_active,
        ErrorCode::BusinessNotActive
    );

    if amount_spent > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new(
                anchor_lang::system_program::ID,
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.business_wallet.to_account_info(),
                },
            ),
            amount_spent,
        )?;
    }

    let mut discount_applied: u8 = 0;
    let mut used_discount = false;
    if use_discount {
        let discount = ctx
            .accounts
            .discount
            .as_ref()
            .ok_or(ErrorCode::Unauthorized)?;
        require!(discount.is_active, ErrorCode::DiscountExpired);
        require!(
            discount.valid_until == 0 || discount.valid_until > timestamp,
            ErrorCode::DiscountExpired
        );
        require!(
            amount_spent >= discount.min_purchase,
            ErrorCode::BelowMinPurchase
        );
        if !discount.unlimited {
            require!(
                discount.available_units > 0,
                ErrorCode::DiscountExhausted
            );
        }
        discount_applied = discount.discount_percent;
        used_discount = true;
        if !discount.unlimited {
            let d = ctx.accounts.discount.as_mut().unwrap();
            d.available_units = d
                .available_units
                .checked_sub(1)
                .ok_or(ErrorCode::DiscountExhausted)?;
            if d.available_units == 0 {
                d.is_active = false;
            }
        }
    }

    let mut xp = BUSINESS_XP_DEFAULT;

    if let Some(mission) = ctx.accounts.mission.as_ref() {
        require!(mission.is_active, ErrorCode::MissionNotActive);
        require!(
            mission.expires_at == 0 || mission.expires_at > timestamp,
            ErrorCode::MissionExpired
        );
        let business_in_mission = mission
            .target_businesses
            .iter()
            .any(|b| b == &ctx.accounts.business_wallet.key());
        require!(business_in_mission, ErrorCode::BusinessNotInMission);

        let progress = ctx
            .accounts
            .mission_progress
            .as_mut()
            .ok_or(ErrorCode::Overflow)?;
        require!(!progress.completed, ErrorCode::MissionAlreadyCompleted);
        progress.user = ctx.accounts.user.key();
        progress.mission = mission.key();
        progress.check_ins = progress
            .check_ins
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        if progress.check_ins >= mission.required_check_ins {
            progress.completed = true;
            xp = xp
                .checked_add(MISSION_BONUS_XP)
                .ok_or(ErrorCode::Overflow)?;
        }
        progress.bump = 0;
    }

    apply_xp(&mut ctx.accounts.passport, xp);

    let business = &mut ctx.accounts.business_account;
    business.rating_sum = business
        .rating_sum
        .checked_add(rating as u64)
        .ok_or(ErrorCode::Overflow)?;
    business.rating_count = business
        .rating_count
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    let check_in = &mut ctx.accounts.check_in;
    check_in.user = ctx.accounts.user.key();
    check_in.business = ctx.accounts.business_wallet.key();
    check_in.timestamp = timestamp;
    check_in.amount_spent = amount_spent;
    check_in.rating = rating;
    check_in.xp_earned = xp;
    check_in.used_discount = used_discount;
    check_in.discount_applied = discount_applied;
    check_in.bump = ctx.bumps.check_in;

    let _ = mission_id;
    msg!(
        "Huellazo: business check-in xp={} rating={} discount={}",
        xp,
        rating,
        discount_applied
    );
    Ok(())
}
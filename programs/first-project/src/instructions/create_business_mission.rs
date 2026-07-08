use crate::{
    constants::*,
    error::ErrorCode,
    state::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateBusinessMission<'info> {
    #[account(mut)]
    pub business_group: Signer<'info>,
    #[account(
        init,
        payer = business_group,
        space = 8 + BusinessMission::INIT_SPACE,
        seeds = [BUSINESS_MISSION_SEED, &id.to_le_bytes()],
        bump
    )]
    pub mission: Account<'info, BusinessMission>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_business_mission(
    ctx: Context<CreateBusinessMission>,
    id: u64,
    name: String,
    required_check_ins: u8,
    target_businesses: Vec<Pubkey>,
    reward_discount: u8,
    expires_at: i64,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, ErrorCode::Overflow);
    require!(
        target_businesses.len() <= MAX_TARGET_BUSINESSES,
        ErrorCode::TooManyTargets
    );
    require!(required_check_ins > 0, ErrorCode::Overflow);

    let mission = &mut ctx.accounts.mission;
    mission.id = id;
    mission.business_group = ctx.accounts.business_group.key();
    mission.name = name;
    mission.required_check_ins = required_check_ins;
    mission.target_businesses = target_businesses;
    mission.reward_discount = reward_discount;
    mission.expires_at = expires_at;
    mission.is_active = true;
    mission.bump = ctx.bumps.mission;
    Ok(())
}
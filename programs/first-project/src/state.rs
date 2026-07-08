use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub backend_admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Passport {
    pub owner: Pubkey,
    pub level: u64,
    pub total_xp: u64,
    pub stamps: u64,
    pub last_spot: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TouristSpot {
    pub id: u64,
    pub authority: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub latitude: i64,
    pub longitude: i64,
    pub radius: u32,
    pub base_xp_reward: u64,
    #[max_len(128)]
    pub achievement_uri: String,
    pub total_visitors: u64,
    pub is_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct GeoAchievement {
    pub user: Pubkey,
    pub spot: Pubkey,
    pub timestamp: i64,
    pub location_proof: [u8; 64],
    pub xp_earned: u64,
    pub shared_on_social: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BusinessAccount {
    pub authority: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub rating_sum: u64,
    pub rating_count: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BusinessMission {
    pub id: u64,
    pub business_group: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub required_check_ins: u8,
    #[max_len(10)]
    pub target_businesses: Vec<Pubkey>,
    pub reward_discount: u8,
    pub expires_at: i64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserBusinessMissionProgress {
    pub user: Pubkey,
    pub mission: Pubkey,
    pub check_ins: u8,
    pub completed: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BusinessCheckIn {
    pub user: Pubkey,
    pub business: Pubkey,
    pub timestamp: i64,
    pub amount_spent: u64,
    pub rating: u8,
    pub xp_earned: u64,
    pub used_discount: bool,
    pub discount_applied: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Discount {
    pub id: u64,
    pub business: Pubkey,
    #[max_len(32)]
    pub code: String,
    pub discount_percent: u8,
    pub min_purchase: u64,
    pub available_units: u32,
    pub unlimited: bool,
    pub valid_until: i64,
    pub is_active: bool,
    pub bump: u8,
}
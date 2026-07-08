pub mod constants;
pub mod error;
pub mod instructions;
pub mod proof;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("88P3XfMfHcnWedUa1u5VJswc6eMQgTKCcBguHBDw6Ztv");

#[program]
pub mod first_project {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, backend_admin: Pubkey) -> Result<()> {
        crate::instructions::init_config::handle_init_config(ctx, backend_admin)
    }

    pub fn update_backend_admin(
        ctx: Context<UpdateBackendAdmin>,
        backend_admin: Pubkey,
    ) -> Result<()> {
        crate::instructions::init_config::handle_update_backend_admin(ctx, backend_admin)
    }

    pub fn initialize_passport(ctx: Context<InitializePassport>) -> Result<()> {
        crate::instructions::initialize_passport::handle_initialize_passport(ctx)
    }

    pub fn create_tourist_spot(
        ctx: Context<CreateTouristSpot>,
        id: u64,
        name: String,
        latitude: i64,
        longitude: i64,
        radius: u32,
        base_xp_reward: u64,
        achievement_uri: String,
    ) -> Result<()> {
        crate::instructions::create_tourist_spot::handle_create_tourist_spot(
            ctx,
            id,
            name,
            latitude,
            longitude,
            radius,
            base_xp_reward,
            achievement_uri,
        )
    }

    pub fn set_spot_active(ctx: Context<SetSpotActive>, active: bool) -> Result<()> {
        crate::instructions::create_tourist_spot::handle_set_spot_active(ctx, active)
    }

    pub fn visit_landmark(
        ctx: Context<VisitLandmark>,
        timestamp: i64,
        location_proof: [u8; 64],
    ) -> Result<()> {
        crate::instructions::visit_landmark::handle_visit_landmark(ctx, timestamp, location_proof)
    }

    pub fn share_achievement(ctx: Context<ShareAchievement>) -> Result<()> {
        crate::instructions::share_achievement::handle_share_achievement(ctx)
    }

    pub fn create_business(ctx: Context<CreateBusiness>, name: String) -> Result<()> {
        crate::instructions::create_business::handle_create_business(ctx, name)
    }

    pub fn set_business_active(ctx: Context<SetBusinessActive>, active: bool) -> Result<()> {
        crate::instructions::create_business::handle_set_business_active(ctx, active)
    }

    pub fn create_business_mission(
        ctx: Context<CreateBusinessMission>,
        id: u64,
        name: String,
        required_check_ins: u8,
        target_businesses: Vec<Pubkey>,
        reward_discount: u8,
        expires_at: i64,
    ) -> Result<()> {
        crate::instructions::create_business_mission::handle_create_business_mission(
            ctx,
            id,
            name,
            required_check_ins,
            target_businesses,
            reward_discount,
            expires_at,
        )
    }

    pub fn create_discount(
        ctx: Context<CreateDiscount>,
        id: u64,
        code: String,
        discount_percent: u8,
        min_purchase: u64,
        available_units: u32,
        valid_until: i64,
    ) -> Result<()> {
        crate::instructions::create_discount::handle_create_discount(
            ctx,
            id,
            code,
            discount_percent,
            min_purchase,
            available_units,
            valid_until,
        )
    }

    pub fn check_in_business(
        ctx: Context<CheckInBusiness>,
        amount_spent: u64,
        rating: u8,
        use_discount: bool,
        discount_id: Option<u64>,
        mission_id: Option<u64>,
        timestamp: i64,
    ) -> Result<()> {
        crate::instructions::check_in_business::handle_check_in_business(
            ctx,
            amount_spent,
            rating,
            use_discount,
            discount_id,
            mission_id,
            timestamp,
        )
    }
}
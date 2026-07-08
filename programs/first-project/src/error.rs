use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid rating, must be between 1 and 5")]
    InvalidRating,
    #[msg("Tourist spot is not active")]
    SpotNotActive,
    #[msg("Discount has expired")]
    DiscountExpired,
    #[msg("Discount has no available units")]
    DiscountExhausted,
    #[msg("Purchase below minimum for discount")]
    BelowMinPurchase,
    #[msg("Business is not active")]
    BusinessNotActive,
    #[msg("Mission has expired")]
    MissionExpired,
    #[msg("Mission is not active")]
    MissionNotActive,
    #[msg("Business is not part of this mission")]
    BusinessNotInMission,
    #[msg("Mission already completed")]
    MissionAlreadyCompleted,
    #[msg("Achievement already shared")]
    AlreadyShared,
    #[msg("Achievement not owned by this user")]
    NotAchievementOwner,
    #[msg("Invalid location proof signature")]
    InvalidLocationProof,
    #[msg("Invalid public key bytes")]
    InvalidPubkeyBytes,
    #[msg("Invalid signature bytes")]
    InvalidSignatureBytes,
    #[msg("Too many target businesses")]
    TooManyTargets,
    #[msg("Numeric overflow")]
    Overflow,
}
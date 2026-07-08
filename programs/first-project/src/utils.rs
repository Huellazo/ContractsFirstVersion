pub fn compute_level(total_xp: u64) -> u64 {
    total_xp / crate::constants::XP_PER_LEVEL
}

pub fn apply_xp(passport: &mut crate::state::Passport, xp: u64) {
    passport.total_xp = passport
        .total_xp
        .checked_add(xp)
        .unwrap_or(passport.total_xp);
    passport.level = compute_level(passport.total_xp);
}

pub fn share_bonus(xp_earned: u64) -> u64 {
    xp_earned
        .checked_mul(crate::constants::SHARE_BONUS_BPS)
        .unwrap_or(0)
        / crate::constants::BPS_DENOMINATOR
}
use anchor_lang::prelude::*;

#[account]
pub struct Vaccine {
    pub pubkey: Pubkey,
    pub batch: Pubkey,
    pub used: bool,
    pub used_by: Option<Pubkey>,
    pub used_at: Option<i64>,
}

impl Vaccine {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1 + (1 + 32) + (1 + 8);
}

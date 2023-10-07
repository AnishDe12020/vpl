use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BatchStatus {
    Manufactured = 0,
    StoredByDistributor = 1,
    ReceivedByDoctor = 2,
}

#[account]
pub struct Batch {
    pub pubkey: Pubkey,
    pub manufacturer: Pubkey,
    pub distributor: Pubkey,
    pub manufactured_at: i64,
    pub expires_at: i64,
    pub quantity: u8,
    pub temp_min: u8,
    pub temp_max: u8,
    pub cost_per_piece: u16,
    pub status: BatchStatus,
    pub temp_defect: bool,
}

impl Batch {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 1 + 2 + 1 + 1;
}

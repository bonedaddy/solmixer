use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Asset {
    Sol, // indicates the mixer supports the SOL token
    Unsupported,
}

impl Asset {
    pub fn is_valid_asset(asset: u64) -> bool {
        match asset {
            0 => true,
            _ => false,
        }
    }
    pub fn from_u64(asset: u64) -> Option<Asset> {
        match asset {
            0 => Some(Asset::Sol),
            _ => None,
        }
    }
}
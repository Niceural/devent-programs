use anchor_lang::prelude::*;

pub mod mint_nft;
pub mod sell_nft;
// pub mod errors;

use mint_nft::*;
use sell_nft::*;
// use errors::Errors;

declare_id!("59sCeP718NpdHv3Xj6kjgrmGNEt67BNXFcy5VUBUDhJE");

#[program]
pub mod devent {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNft>,
        // supply: u64,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        mint_nft::mint_nft(
            ctx,
            // supply,
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )
    }

    pub fn sell_nft(
        ctx: Context<SellNft>,
        sale_lamports: u64,
    ) -> Result<()> {
        sell_nft::sell_nft(
            ctx,
            sale_lamports,
        )
    }
}
// import {useState} from "react";

use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash, program::invoke, system_instruction::transfer},
};

mod constants;
use crate::constants::*;
// getting into contansts and grabbing everything here

declare_id!("6MBkRMjXVBpTJtUSfRmiG9csyAL6bJ8JXfJzmu44yBtE");
// program id

#[program]

mod lottery {

    use super::*;
    // using all from above

    pub fn init_master(_ctx: Context<InitMaster>) -> Result<()> {
        // write logic here
        // what is the master -> an object that gonna hold last lottery id

        Ok(())
    }

}

#[derive(Accounts)]

pub struct InitMaster<'info> {
    #[account(
        init,
        // account init
        payer= payer,
        space = 4 + 8,
        seeds = [MASTER_SEED.as_bytes()],
        bump
     )]
    pub master: Account<'info, Master>,
    //  macro initilize a master account
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}

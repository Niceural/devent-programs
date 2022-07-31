use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::collections::HashMap;

declare_id!("59sCeP718NpdHv3Xj6kjgrmGNEt67BNXFcy5VUBUDhJE");

#[program]
pub mod devent {
    use super::*;

    /// Initializes the state of the program. 
    /// This function must be called before any event is created.
    /// It declares a variable used to create an event.
    pub fn create_state(
        ctx: Context<CreateState>,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.event_count = 0;
        Ok(())
    }

    /*
    pub fn create_event(
        ctx: Context<CreateEvent>,
        max_attendees: u64,
        min_price: u64,
    ) -> Result<()> {
        // get state
        let state = &mut ctx.accounts.state;
        // get event
        let event = &mut ctx.accounts.event;
        event.authority = ctx.accounts.authority.key();
        event.event_id = state.event_count;
        event.max_attendees = max_attendees;
        event.number_of_attendees = 0;
        event.min_price = min_price;

        state.event_count += 1;
        Ok(())
    }

    pub fn attendee_registers(
        ctx: Context<AttendeeRegisters>,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        if event.number_of_attendees >= event.max_attendees {
            return Err(ErrorCode::MaxCapacity.into())
        }
        event.number_of_attendees += 1;
        event.attendees.insert(ctx.accounts.authority.key(), true);
        Ok(())
    }
    */
}

#[derive(Accounts)]
pub struct CreateState<'info> {
    // authenticating state account
    #[account(
        init,
        // seeds = [b"state".as_ref()],
        // bump,
        payer = authority,
        space = StateAccount::LEN,
    )]
    pub state: Account<'info, StateAccount>,
    // authority (signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Token program (no clue what it is)
    // #[account(constraint = token_program.key == &token::ID)]
    // pub token_program: Program<'info, Token>,
}

/*
#[derive(Accounts)]
pub struct CreateEvent<'info> {
    // authenticate state account
    #[account(
        mut,
        seeds = [b"state".as_ref()],
        bump,
    )]
    pub state: Account<'info, StateAccount>,

    // authenticate event account
    #[account(
        init,
        seeds = [b"event".as_ref(), state.event_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = EventAccount::LEN,
    )]
    pub event: Account<'info, EventAccount>,
    // authority (signer who pays transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,
    /// System program
    pub system_program: UncheckedAccount<'info>,
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AttendeeRegisters<'info> {
    #[account(
        mut,
        seeds = [b"event".as_ref(), event.event_id.to_be_bytes().as_ref()],
        bump
    )]
    pub event: Account<'info, EventAccount>,

    // Authority (signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}
*/

#[account]
pub struct StateAccount {
    pub authority: Pubkey, // signer address
    pub event_count: u64, // number of events
}

impl StateAccount {
    const LEN: usize = 32 + 64;
}

/*
#[account]
pub struct Event{
    pub authority: Pubkey, // event organizer
    pub index: u64, // given by the StateAccount
    pub max_registered: u64, // maximum number of Pubkeys allowed to register
    pub amount_registered: u64, // amount of Pubkeys currently registered
    pub min_lamports: u64, // minimum registration price in lamports
    pub attendees: HashMap<Pubkey, Status>, // mapping of PubKeys to not registered, registered, attended
}

impl Event {
    const LEN: usize = 32 + 64 + 64 + 64 + 64 + size_of::<HashMap<Pubkey, Status>>();
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Status {
    NotRegistered,
    Registered,
    AttendanceVerified,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Event at maximum capacity")]
    MaxCapacity,
}
*/
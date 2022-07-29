use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
// use anchor_lang::solana_program::blake3::Hash;
use std::collections::HashMap;
use std::mem::size_of;

declare_id!("59sCeP718NpdHv3Xj6kjgrmGNEt67BNXFcy5VUBUDhJE");

#[program]
pub mod devent {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>, 
        event_id: u64,
        max_attendees: u64, 
        // min_price: u64
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.organizer = ctx.accounts.organizer.key();
        event.event_id = event_id;
        event.max_attendees = max_attendees;
        // event.min_price = min_price;
        Ok(())
    }

    pub fn organizer_registers_attendee(
        ctx: Context<OrganizerRegistersAttendee>,
        attendee: Pubkey,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.attendees.insert(attendee, true);
        Ok(())
    }

    pub fn attendee_registers(
        ctx: Context<AttendeeRegisters>,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let attendee = ctx.accounts.attendee.key();
        event.attendees.insert(attendee, true);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer=organizer, space=Event::LEN)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    /// CHECK no idea what that does
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
    // check what token_program is
}

#[derive(Accounts)]
pub struct OrganizerRegistersAttendee<'info> {
    #[account(mut, has_one=organizer)]
    pub event: Account<'info, Event>,
    pub organizer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AttendeeRegisters<'info> {
    #[account(mut)]
    pub event: Account<'info, Event>,
    pub attendee: Signer<'info>,
}

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub event_id: u64,
    pub max_attendees: u64,
    pub min_price: u64,
    pub attendees: HashMap<Pubkey, bool>,
}

impl Event {
    const LEN: usize = 32 + 64 + 64 + 64 + size_of::<HashMap<Pubkey, bool>>();
}

#[error_code]
pub enum ErrorCode {
    #[msg("Incorrect organizer")]
    IncorrectOrganizer,
    #[msg("Event at maximum capacity")]
    MaxCapacity,
    #[msg("Insufficient amount sent")]
    InsufficientAmount
}
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("GWS62PHUozZsjJWDZi7tKmwmRkazVVCkFzv6kPZ87uHq");

#[program]
pub mod participation_program {
    use super::*;

    pub fn initialize_participant(ctx: Context<InitializeParticipant>) -> Result<()> {
        let participant = &mut ctx.accounts.participant;
        participant.chip_count = 0;
        participant.authority = *ctx.accounts.player.key;

        msg!("Participant initialized!");
        msg!("Participant data: {:?}", participant);

        Ok(())
    }

    pub fn get_game_chip(ctx: Context<GetGameChip>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.player_token_account.to_account_info(),
            to: ctx.accounts.game_token_account.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        ctx.accounts.participant.chip_count += 1;

        msg!("Game chip acquired!");
        msg!("Participant data: {:?}", ctx.accounts.participant);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeParticipant<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init,
        seeds = [b"participant", player.key().as_ref()],
        bump,
        payer = player,
        space = 8 + 8 + 32, // 8 bytes for discriminator, 8 bytes for u64, 32 bytes for Pubkey
    )]
    pub participant: Account<'info, Participant>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetGameChip<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub game_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"participant", player.key().as_ref()],
        bump,
    )]
    pub participant: Account<'info, Participant>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug)]
pub struct Participant {
    pub authority: Pubkey,
    pub chip_count: u64,
}

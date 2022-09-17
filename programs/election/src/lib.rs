use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
#[program]
pub mod election {
    use super::*;
    pub fn create_election(ctx: Context<CreateElection>,winners:u8) -> Result<()> {
        require!(winners > 0,ElectionError::WinnerCountNotAllowed);
        let election = &mut ctx.accounts.election_data;
        election.candidates = 0; 
        election.stage = ElectionStage::Application;
        election.initiator = ctx.accounts.signer.key();
        election.winners_num = winners;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(winners:u8)]
pub struct CreateElection<'info> {
    #[account(
        init,
        payer=signer,
        space= 8 + 8 + 2 + 32 + 1 + 2 * (4 + winners as usize * 8)
    )]
    pub election_data: Account<'info,ElectionData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
pub struct ElectionData {
    pub candidates: u64,
    pub stage: ElectionStage,
    pub initiator: Pubkey,
    pub winners_num: u8,
    pub winners_id: Vec<u64>,
    pub winners_votes: Vec<u64>,
}

#[derive(AnchorDeserialize,AnchorSerialize,PartialEq,Eq,Clone)]
pub enum ElectionStage {
    Application,
    Voting,
    Closed,
}

#[error_code]
pub enum ElectionError {
    WinnerCountNotAllowed
}
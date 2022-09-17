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

    pub fn apply(ctx: Context<Apply>) -> Result<()> {
        let election = &mut ctx.accounts.election_data;
    
        require!(election.stage == ElectionStage::Application,ElectionError::ApplicationIsClosed);
    
        election.candidates += 1;
        ctx.accounts.candidate_identity.id = election.candidates;
        ctx.accounts.candidate_identity.pubkey = ctx.accounts.signer.key();
        Ok(())
    }
    
    pub fn register(ctx: Context<Register>) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_data;
    
        candidate.votes = 0;
        candidate.pubkey = ctx.accounts.signer.key();
        candidate.id = ctx.accounts.candidate_identity.id;
    
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

#[derive(Accounts)]
pub struct Apply<'info> {
    #[account(
        init,
        payer=signer,
        space=8+8+32,
        seeds=[
            b"candidate",
            signer.key().as_ref(),
            election_data.key().as_ref()
        ],
        bump
    )]
    pub candidate_identity: Account<'info,CandidateIdentity>,
    #[account(mut)]
    pub election_data: Account<'info,ElectionData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        payer=signer,
        space=8+8+8+32,
        seeds=[
            &(candidate_identity.id).to_be_bytes(),
            election_data.key().as_ref()
        ],
        bump
    )]
    pub candidate_data: Account<'info,CandidateData>,
    pub election_data: Account<'info,ElectionData>,
    pub candidate_identity: Account<'info,CandidateIdentity>,
    #[account(mut,address=candidate_identity.pubkey @ ElectionError::WrongPublicKey)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
pub struct CandidateData {
    pub votes: u64,
    pub id: u64,
    pub pubkey: Pubkey,
}

#[account]
pub struct CandidateIdentity {
    pub id: u64,
    pub pubkey: Pubkey,
}




#[derive(AnchorDeserialize,AnchorSerialize,PartialEq,Eq,Clone)]
pub enum ElectionStage {
    Application,
    Voting,
    Closed,
}

#[error_code]
pub enum ElectionError {
    WinnerCountNotAllowed,
    ApplicationIsClosed,
    WrongPublicKey,
}
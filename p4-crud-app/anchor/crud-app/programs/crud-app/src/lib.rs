use anchor_lang::prelude::*;

declare_id!("14odPdmodnGEtyRtpFhpkZbceU1TP7hpwUzFVF1aCt3k");

#[program]
pub mod crud_app {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>,
    title : String,
    message : String) -> Result<()> {
        msg!("Creating Journal Entry with following details");
        msg!("Title : {}",title);
        msg!("Message : {}",message);
        
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

     pub fn update_journal_entry(ctx: Context<CreateEntry>,
    title : String,
    message : String) -> Result<()> {
        msg!("Updatting Journal Entry with following details");
        msg!("Title : {}",title);
        msg!("Message : {}",message);
        
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, title: String) -> Result<()> {
        msg!("Journal entry titled {} deleted", title);
        Ok(())    
    }

}

#[derive(Accounts)]
pub struct Initialize {}


#[account]
pub struct JournalEntryState {
    pub owner : Pubkey,
    pub title : String,
    pub message : String,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {

    #[account(
        init,
        seeds = [title.as_bytes(),owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8+32+4+title.len()+4+message.len()
    )]
    pub journal_entry: Account<'info,JournalEntryState>,
    #[account(mut)]
    pub owner : Signer<'info>,
    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
#[instruction(title:String,message:String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(),owner.key().as_ref()],
        bump,
        close=owner,
    )]
    pub journal_entry:Account<'info,JournalEntryState>,
    #[account(mut)]
    pub owner : Signer<'info>,
    pub system_program:Program<'info,System>,
}


#[derive(Accounts)]
#[instruction(title:String,message:String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(),owner.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 +title.len()+4+message.len(),
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub journal_entry:Account<'info,JournalEntryState>,
    #[account(mut)]
    pub owner : Signer<'info>,
    pub system_program:Program<'info,System>,
}


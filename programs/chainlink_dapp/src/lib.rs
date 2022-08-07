use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::system_program;
declare_id!("BLTPnDNGKsxmyumWcMMAuvrygPwXYbswanTvW2Zjjja7");
use chainlink_solana as chainlink;

#[program]
pub mod chainlink_dapp {
    use super::*;

    pub fn execute(ctx:Context<Execute>) -> ProgramResult {
        let round = chainlink::latest_round_data(
        ctx.accounts.chainlink_program.to_account_info(),
        ctx.accounts.chainlink_feed.to_account_info())?;

        let result_account = &mut ctx.accounts.result_account;
        result_account.value = round.answer;
        Ok(())
        }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(init, payer=user, space = 100)]
    pub result_account: Account<'info, ResultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub chainlink_program:AccountInfo<'info>,
    /// CHECK:
    pub chainlink_feed: AccountInfo<'info>
}


#[account]
pub struct ResultAccount {
    pub value: i128
}





use {
    crate::objects::*,
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(settings: NodeSettings)]
pub struct NodeUpdate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_NODE,
            node.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
    )]
    pub node: Account<'info, Node>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<NodeUpdate>, settings: NodeSettings) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let node = &mut ctx.accounts.node;
    let system_program = &ctx.accounts.system_program;

    // Update the node
    node.update(settings)?;

    // Realloc memory for the node account
    let data_len = 8 + node.try_to_vec()?.len();
    node.to_account_info().realloc(data_len, false)?;

    // If lamports are required to maintain rent-exemption, pay them
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > node.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: node.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(node.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}

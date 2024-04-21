use anchor_lang::prelude::*;

declare_id!("A41xLru1oTy3mZuZF88MA2DQZ76vV7gTCKFvS4RWyumw");

#[program]
pub mod anchor_v030 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

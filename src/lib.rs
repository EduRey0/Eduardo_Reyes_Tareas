use anchor_lang::prelude::*;

declare_id!("ZTQeaHP7DwzyBAovQ9RLJAhL5h4auL3oSUv1eyWXbFe");

#[program]
pub mod crud_app {
    use super::*;

    pub fn crear(ctx: Context<Crear>, id: u64, titulo: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.owner = ctx.accounts.owner.key();
        task.id = id;
        task.titulo = titulo;
        task.completado = false;
        Ok(())
    }

    pub fn actualizar(
        ctx: Context<Actualizar>,
        nuevo_titulo: String,
        estado: bool,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;

        require!(
            task.owner == ctx.accounts.owner.key(),
            Errores::NoAutorizado
        );

        task.titulo = nuevo_titulo;
        task.completado = estado;
        Ok(())
    }

    pub fn eliminar(_ctx: Context<Eliminar>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Task {
    pub owner: Pubkey,
    pub id: u64,
    pub titulo: String,
    pub completado: bool,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Crear<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4 + 100 + 1,
        seeds = [
            b"task2",
            owner.key().as_ref(),
            &id.to_le_bytes()
        ],
        bump
    )]
    pub task: Account<'info, Task>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Actualizar<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub task: Account<'info, Task>,
}

#[derive(Accounts)]
pub struct Eliminar<'info> {
    #[account(mut, close = owner)]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[error_code]
pub enum Errores {
    #[msg("No autorizado")]
    NoAutorizado,
}
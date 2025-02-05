use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey, worker_pool: Pubkey) -> Instruction {
    let config_pubkey = clockwork_queue_program::objects::Config::pubkey();
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_queue_program::instruction::Initialize { worker_pool }.data(),
    }
}

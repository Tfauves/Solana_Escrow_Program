// Someone calls the entrypoint
// The entrypoint forwards the arguments to the processor
// The processor asks instruction.rs to decode the instruction_data argument from the entrypoint function.
// Using the decoded data, the processor will now decide which processing function to use to process the request.
// The processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint.

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::Processor;
// entrypoint! is a MACRO* that allows us to call a program, all calls go through the function declared as entrypoint
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}

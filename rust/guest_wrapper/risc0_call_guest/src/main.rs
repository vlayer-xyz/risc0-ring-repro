#![no_main]

risc0_zkvm::guest::entry!(main);

fn main() {
    call_guest::main();
}

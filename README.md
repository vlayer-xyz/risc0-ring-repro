# Risc0 ring reproduction

RSA in ring does not work on newer risc0 on Bonsai.

In this repo I have a small host/guest pair. You can generate fake proofs and bonsai proofs using the following commands:

Groth16 Bonsai proofs
```rs
BONSAI_API_URL=https://api.bonsai.xyz/ \
BONSAI_API_KEY=$BONSAI_KEY \
cargo run
```

Fake proofs
```rs
RISC0_DEV_MODE=1 cargo run
```

If you run it now (on risc0 1.2.4) then fake proofs succeed but Bonsai fails.
If you downgrade risc0 to 1.2.0 (remember to do it in both `.` and `./rust/guest_wrapper/risc0_call_guest`) - it works both bonsai and fake.

I expect bonsai to also succeed if local fake proof succeeds
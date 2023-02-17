**Draft of the modular-dao's smart contracts.**

# Project structure:
```bash
.
├── Cargo.lock
├── Cargo.toml
├── contracts       <- example contracts
├── impls           <- the "default implementation of the contracts"
├── lib.rs
├── README.md
└── traits          <- the "interfaces of the contracts"
```
# How does it works atm?:
* It is a multi-contract* system where the "Master" DAO contract stores refernces to all it's modules i.e. strategies, proposal_types and maybe more in the future.
* Once the "Master" Contract is deployed, the "module" contracts have to be deployed as well with "Master" contract address passed to constructor.
* The "Master" DAO has add these modules by calling appropriate methods.
<br /> 
\* it is possible that a single contract implements all the traits i.e. master_dao, strategy, proposal. However, it such a case it is limited to have only "one kind" of each "module".
<br />
# TODOs:
## General
- [x] switch to ink! 4.0-rc
- [ ] tests

## DAO
- [x] vote delegation system
- [x] "liberum veto"
- [ ] private voting
## Strategies:
- [ ] define 2 - 3 functions for vote weight computation e.i. charmonic mean, simple factor multiplication, log function etc.
- [x] gov22 strategy
- [ ] gov34 strategy
- [ ] gov37 strategy
- [x] whitelist strategy

## Proposals:
- [x] transfer_asset
- [x] no_action

## Scheduler
* ...

## Treasury
* ...
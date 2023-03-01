<h1 align="center">
    LiberumDAO's implementation of modular-dao concept.
</h1>

## What is it?

Modular-dao is a set of smart contracts built using [ink!](https://github.com/paritytech/ink) and [OpenBrush](https://github.com/727-Ventures/openbrush-contracts) that allows creation of customizable DAOs.

## How does it work?

At the moment, the system consists of 3 types of modules: **DAO-base**, **strategy** and **proposal**. 
* **DAO-base** role is to be a "master" of all the other smart contracts. It defines the basic rules of a DAO, implements a solution for vote delegation, role-based governance of the DAO and interacts with other smart contracts that implement other logic required in a DAO.
* The **strategy** is responsible for calculating members' "influence" in a DAO. The example smart contracts that implement **strategy** trait are PSP22 based strategy and whitelist strategy. 
* The **proposal** module brings the proposal creation, voting, and proposal execution functionalities. The example smart contract that implements **proposal** allows to transfer specified amount of native token to an account and emits appropriate event upon the proposal execution.

## Modularity



## Project structure:
```bash
.
├── Cargo.lock
├── Cargo.toml
├── contracts       <- example smart contracts built using traits
├── impls           <- the default implementation of the traits
├── lib.rs
├── README.md
└── traits          <- the traits ("interfaces") of the smart contracts
```
## How does it works at the moment?
* It is a multi-contract* system where the "Master" DAO contract stores refernces to all it's modules i.e. strategies, proposal_types and maybe more in the future.
* Once the "Master" Contract is deployed, the "module" contracts have to be deployed as well with "Master" contract address passed to constructor.
* The "Master" DAO has to add these modules by calling appropriate methods.

(*) it is possible that a single contract implements all the traits i.e. master_dao, strategy, proposal. However, in such a case it is limited to have only "one kind" of each "module".

## License

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

---
---
## TODOs:
### General
- [x] switch to ink! 4.0-rc
- [ ] tests

### DAO
- [x] vote delegation system
- [x] "liberum veto"
- [ ] private voting
### Strategies:
- [ ] define 2 - 3 functions for vote weight computation e.i. charmonic mean, simple factor multiplication, log function etc.
- [x] gov22 strategy
- [ ] gov34 strategy
- [ ] gov37 strategy
- [x] whitelist strategy

### Proposals:
- [x] transfer_asset
- [x] no_action

### Scheduler
* ...

## Treasury
* ...


<div align="center">
    <img src="https://liberumdao.io/logo.svg" alt="ink!" height="136"/>
<h1 align="center">
    LiberumDAO's implementation of modular-dao concept.
</h1>
</div>

## What is it?

Modular-dao is a set of smart contracts built using [ink!](https://github.com/paritytech/ink) and [OpenBrush](https://github.com/727-Ventures/openbrush-contracts) that allows creation of customizable DAOs.

## How does it work?

At the moment, the system consists of 3 types of modules: **DAO-base**, **strategy** and **proposal**. 
* **DAO-base** role is to be a "master" of all the other smart contracts. It defines the basic policies of a DAO, implements a solution for vote delegation, role-based governance of the DAO and interacts with other smart contracts that implement other logic required in the DAO.
* The **strategy** is responsible for calculating members' "influence" in a DAO. The example smart contracts that implement **strategy** trait are PSP22 based strategy and whitelist strategy. 
* The **proposal** module brings the proposal creation, voting, and proposal execution functionalities. The example smart contract that implements **proposal** allows to transfer specified amount of native token to an account and emits appropriate event upon the proposal execution.

In the future, we plan to develop more modules such as private voting and multi-chian treasury to increase functionality and customizability. 

### **Strategy**
All the contracts that implement **strategy** share common interface. However, the logic that is responsible for calculating "influence" can be defined independently. This combined with modular approach, allows a DAO to incorporate multiple strategies in parallel.

### **Proposal**
The proposal lifetime consists of 4 cycles: creation, voting period, counting votes and execution. The creation and voting period are straight-forward and self-explanatory. After a proposal is created and submitted by an user with appropriate role in DAO, users can vote on proposal using one out of 3 options: *For*, *Against* or *Abstain*. After the voting period passes, it is not possible to submit a vote. After that, the votes are counted by calling appropriate method on **proposal** contract. The votes weights are calculated based on the incorporated strategies in DAO. It is important to notice that the "influence" used to calculate votes weights is considered during the vote calculation cycle, not during the voting period. Finally, depending on the outcome, the proposal is either rejected or set to be pedning for execution.

At the moment, the implemented **proposal** contract's execution logic is to transfer certain amount of native token to specified wallet. In the future, we plan to seperate the "execution logic" from the proposal so it is possible to enact custom actions upon propsal execution.


## How to try it out?
To compile the contracts, you need to use ```cargo-contract v2.0.0```. You can install it as follows:
```cargo install cargo-contract --forced --locked```

### Deployment
First, you have to deploy **DAO-base** contract. In the constructor specify what wallets should be considered as ```FOUNDER```s of the DAO (the caller is ```FOUNDER``` by default). Then deploy desired module contracts - in the constructors pass the address of the deployed **DAO-base** contract. Finally, add the deployed modules to the DAO by calling appropriate methods on the deployed **DAO-base** contract:
```rust
dao::add_strategy(address: AccountId)
dao::add_proposal_type(address: AccountId)
```
Only wallets that are ```FOUNDER```s of the DAO are allowed to add and remove modules.

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


## License
The entire code within this repository is licensed under the [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0).



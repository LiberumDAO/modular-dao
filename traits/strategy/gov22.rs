use super::*;
#[openbrush::trait_definition]
pub trait GOV22 {
    //TODO
    #[ink(message)]
    fn get_token_address(&self) -> AccountId;
}

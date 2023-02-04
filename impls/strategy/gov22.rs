pub use crate::impls::strategy::*;
use crate::traits::strategy::gov22::*;

impl<T: Storage<Data<AccountId>>> GOV22 for T {
    ///Default implementation of the `modular_dao::traits::DaoMaster`
    default fn get_token_address(&self,) -> AccountId {
        self.data().data
    }

}

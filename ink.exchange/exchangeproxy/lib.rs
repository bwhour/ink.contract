#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod exchangeproxy {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Exchangeproxy {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }
    pub struct Swap {
        pool :   AccountId,
        tokenInParam:  u32, // tokenInAmount / maxAmountIn / limitAmountIn
        tokenOutParam: u32, // mßinAmountOut / tokenAmountOut / limitAmountOut
        maxPrice:      u32,
    }

    #[ink(event+anonymous)]
    pub struct LOG_CALL {
        #[ink(topic)]
        sig: Option<AccountId>,
        #[ink(topic)]
        caller: Option<AccountId>,
        data: [u8; 32],
    }
    impl Exchangeproxy {


        // pub struct LOG_CALL(
        // bytes4  indexed sig,
        // address indexed caller,
        // bytes          data
        // )
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        fn add(a:u32, b:u32) -> u32 {
         let c = a + b;
         //todo assert_eq!(c >= a, "ERR_ADD_OVERFLOW");
         c
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let exchangeproxy = Exchangeproxy::default();
            assert_eq!(exchangeproxy.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut exchangeproxy = Exchangeproxy::new(false);
            assert_eq!(exchangeproxy.get(), false);
            exchangeproxy.flip();
            assert_eq!(exchangeproxy.get(), true);
        }
    }
}

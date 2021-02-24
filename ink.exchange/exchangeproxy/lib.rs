#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod exchangeproxy {
    use token::TokenInterface;
    use ink_env::call::FromAccountId;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::{HashMap as StorageHashMap, Vec as StorageVec},
        traits::{PackedLayout, SpreadLayout},
        Lazy,
    };
    #[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    scale::Encode,
    scale::Decode,
    SpreadLayout,
    PackedLayout,
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Exchangeproxy {
        /// Stores a single `bool` value on the storage.
        _mutex: bool,
        weth: Laze<TokenInterface>,
    }
    pub struct Swap {
        pool :   AccountId,
        tokenInParam:  u32, // tokenInAmount / maxAmountIn / limitAmountIn
        tokenOutParam: u32, // mßinAmountOut / tokenAmountOut / limitAmountOut
        maxPrice:      u32,
    }

    #[ink(event)]
    pub struct LOGCALL {
        #[ink(topic)]
        sig: [u8; 4],
        #[ink(topic)]
        caller: Option<AccountId>,
        data: [u8; 32],
    }
    impl Exchangeproxy {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            init_value: i32,
            mutex:bool,
            weth_code_hash: Hash,
        ) -> Self {
            let total_balance = Self::env().balance();
            let token = TokenInterface::new(init_value)
                .endowment(total_balance / 4)
                .code_hash(weth_code_hash)
                .instantiate()
                .expect("failed at instantiating the `TokenInterface` contract");
            Self {
                _mutex :mutex,
                weth: Lazy::new(token),
            }
        }
        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self { Self::new(Default::default()) }
        #[ink(message)]
        pub fn batchSwapExactIn(
            &mut self,
            swaps : Vec<Swap>,
            tokenIn : AccountId,
            tokenOut : AccountId,
            totalAmountIn : u32,
            minTotalAmountOut : u32,
        ) -> u32 {
            // TokenInterface TI = TokenInterface(tokenIn);
            // TokenInterface TO = TokenInterface(tokenOut);
            let TI:TokenInterface = FromAccountId::from_account_id(tokenIn);
            let TO:TokenInterface = FromAccountId::from_account_id(tokenOut);
            assert_eq!(TI.transfer_from(self.env().caller(),this, totalAmountIn),false);
            let swap: Vec<_> = swaps.iter().copied().collect();
            for x in swap.clone().into_iter() {
                let pool : PoolInterface = PoolInterface::from_account_id(x.pool);
                if TI.allowance(self.env().account_id(), x.pool) < totalAmountIn {
                    TI.approve(x.pool, -1);
                }
                let tokenAmountOut = pool.swapExactAmountIn(
                    tokenIn,
                    x.tokenInParam,
                    tokenOut,
                    x.tokenOutParam,
                    x.maxPrice,
                );
                totalAmountOut = add(tokenAmountOut, totalAmountOut);
            }

            totalAmountOut
        }


        #[ink(message)]
        pub fn batchSwapExactOut (
            &mut self,
            swaps : Vec<Swap>,
            tokenIn : AccountId,
            tokenOut : AccountId,
            maxTotalAmountIn : u32,
        ) -> u32 {
            // TokenInterface TI = TokenInterface(tokenIn);
            // TokenInterface TO = TokenInterface(tokenOut);
            let TI:TokenInterface = FromAccountId::from_account_id(tokenIn);
            let TO:TokenInterface = FromAccountId::from_account_id(tokenOut);
            assert_eq!(TI.transfer_from(self.env().caller(),this, maxTotalAmountIn),false);
            let swap: Vec<_> = swaps.iter().copied().collect();
            for x in swap.clone().into_iter() {
                let pool : PoolInterface = PoolInterface::from_account_id(x.pool);
                if TI.allowance(self.env().account_id(), x.pool) < maxTotalAmountIn {
                    TI.approve(x.pool, -1);
                }
                let tokenAmountIn = pool.swapExactAmountIn(
                    tokenIn,
                    x.tokenInParam,
                    tokenOut,
                    x.tokenOutParam,
                    x.maxPrice,
                );
                totalAmountIn = add(tokenAmountIn, totalAmountIn);
            }

            totalAmountIn
        }

        #[ink(message)]
        pub fn batchEthInSwapExactIn(
            &mut self,
            swaps : Vec<Swap>,
            tokenOut : AccountId,
            minTotalAmountOut : u32,
        ) -> u32 {
            let TO:TokenInterface = FromAccountId::from_account_id(tokenOut);
            // weth.deposit.value(self.env().balance())();
            let swap: Vec<_> = swaps.iter().copied().collect();
            for x in swap.clone().into_iter() {
                let pool : PoolInterface = PoolInterface::from_account_id(x.pool);
                if weth.allowance(self.env().account_id(), x.pool) < self.env().balance() {
                    weth.approve(x.pool, -1);
                }
                let tokenAmountOut = pool.swapExactAmountIn(
                    weth,
                    x.tokenInParam,
                    tokenOut,
                    x.tokenOutParam,
                    x.maxPrice,
                );
                totalAmountOut = add(tokenAmountOut, totalAmountOut);
            }
            let wethBalance = weth.balanceOf(self.env().account_id());
            if wethBalance > 0 {
                weth.withdraw(wethBalance);
                // (bool xfer,) = msg.sender.call.value(wethBalance)("");
                // require(xfer, "ERR_ETH_FAILED");
            }
            totalAmountOut
        }

        #[ink(message)]
        pub fn batchEthOutSwapExactIn(
            &mut self,
            swaps : Vec<Swap>,
            tokenIn : AccountId,
            totalAmountIn : u32,
            minTotalAmountOut : u32,
        ) -> u32 {
            let TI:TokenInterface = FromAccountId::from_account_id(tokenIn);
            assert_eq!(TI.transfer_from(self.env().caller(),this, totalAmountIn),false);
            let swap: Vec<_> = swaps.iter().copied().collect();
            for x in swap.clone().into_iter() {
                let pool : PoolInterface = PoolInterface::from_account_id(x.pool);
                if (TI.allowance(self.env().account_id(), x.pool) < maxTotalAmountIn) {
                    TI.approve(x.pool, -1);
                }
                let tokenAmountOut = pool.swapExactAmountIn(
                    tokenIn,
                    x.tokenInParam,
                    weth,
                    x.tokenOutParam,
                    x.maxPrice,
                );
                totalAmountOut = add(tokenAmountOut, totalAmountOut);
            }
            assert!(totalAmountOut>=minTotalAmountOut);
           let wethBalance = weth.balanceOf(self.env().account_id());
            weth.withdraw(wethBalance);
            // (bool xfer,) = msg.sender.call.value(wethBalance)("");
            // require(xfer, "ERR_ETH_FAILED");
            assert!(TI.transfer(self.env().caller(),TI.balanceOf(self.env().account_id())));
            totalAmountOut
    }
        #[ink(message)]
        pub fn batchEthInSwapExactOut(
            &mut self,
            swaps : Vec<Swap>,
            tokenOut : AccountId,
        ) -> u32 {
            let TO:TokenInterface = FromAccountId::from_account_id(tokenOut);
            weth.deposit.value(Default.default());
            let swap: Vec<_> = swaps.iter().copied().collect();
            for x in swap.clone().into_iter() {
                let pool : PoolInterface = PoolInterface::from_account_id(x.pool);
                if TO.allowance(self.env().account_id(), x.pool) < self.env().balance() {
                    TO.approve(x.pool, -1);
                }
                let tokenAmountIn = pool.swapExactAmountOut(
                    weth,
                    x.tokenInParam,
                    tokenOut,
                    x.tokenOutParam,
                    x.maxPrice,
                );
                totalAmountIn = add(tokenAmountIn, totalAmountIn);
                assert_eq!(TO.transfer(self.env().caller(),TO.balanceof(self.env().account_id())),false);
                let wethBalance = weth.balanceof(self.env().account_id());
                if wethBalance>0 {
                    weth.withdraw(wethBalance);
                    // (bool xfer,) = msg.sender.call.value(wethBalance)("");
                    // assert_eq!(xfer,false);
                }
            }
            totalAmountIn
        }

        #[ink(message)]
        pub fn batchEthOutSwapExactOut(
            &mut self,
            swaps : Vec<Swap>,
            tokenIn : AccountId,
            maxTotalAmountIn : u32,
        ) -> u32 {
            let TI:TokenInterface = FromAccountId::from_account_id(tokenIn);
            assert_eq!(TI.transfer_from(self.env().caller(),this, totalAmountIn),false);
            let swap: Vec<_> = swaps.iter().copied().collect();
            for x in swap.clone().into_iter() {
                let pool : PoolInterface = PoolInterface::from_account_id(x.pool);
                if TI.allowance(self.env().account_id(), x.pool) < maxTotalAmountIn {
                    TI.approve(x.pool, -1);
                }
                let tokenAmountIn = pool.swapExactAmountOut(
                    tokenIn,
                    x.tokenInParam,
                    weth,
                    x.tokenOutParam,
                    x.maxPrice,
                );
                totalAmountIn = add(tokenAmountIn, totalAmountIn);
            }
            assert!(maxTotalAmountIn<=maxTotalAmountIn);
            assert!(TI.transfer(self.env().caller(),TI.balanceOf(self.env().account_id())));
            let wethBalance = weth.balanceOf(self.env().account_id());
            weth.withdraw(wethBalance);
            // (bool xfer,) = msg.sender.call.value(wethBalance)("");
            // assert!(xfer);
            totalAmountIn
        }


        //............................................
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
        fn _logs_(&mut self) {
            // emit LOG_CALL(msg.sig, msg.sender, msg.data);
            let caller = self.env().caller();
            self.env().emit_event();
        }
        fn _locks_(&mut self){
            assert_eq!(self._mutex,false);
            self._mutex =true;
        }

        fn _unlocks_(&mut self){
            self._mutex = false;
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

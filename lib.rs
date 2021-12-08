#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `i32` value on the storage.
        value: i32,
        // Also can use lazy storage for storing a single `i32` value.
        // my_number: ink_storage::Lazy<u32>,
        my_value: ink_storage::collections::HashMap<AccountId, i32>,
    }

    impl Incrementer {
        /// Constructor that initializes the `i32` value and my value `HashMap` to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                /// Lazy storage for initial values
                // my_number: ink_storage::Lazy::<u32>::new(init_value),
                my_value: ink_storage::collections::HashMap::new(),
            }
        }

        /// Constructor that initializes the value `i32` value to `0`.
        /// Constructor that initializes my_value `i32` value to `default HashMap`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Default::default(),
            }
        }

        /// Simply returns the current value of our `i32`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        /// Increments the value of our value `i32` by given number.
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        /// Returns the value of our value `i32` by given caller account id.
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value_or_zero(&self.env().caller())
            // lazy storage update old value to new value
            // ink_storage::Lazy::<u32>::set(&mut self.my_number, new_value)
        }

        /// Increments the value of our value `i32` by given number by caller account id.
        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            let my_value = self.my_value_or_zero(&caller);
            self.my_value.insert(caller, my_value + by);
            // Lazy storage get and set.
            // let cur = ink_storage::Lazy::<u32>::get(my_number);
            // ink_storage::Lazy::<u32>::set(my_number, cur + add_value);
        }

        /// Returns the value of our value `i32` by given caller account id.
        /// If the caller account id is not found, returns `0`.
        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            *self.my_value.get(of).unwrap_or(&0)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        /// We test if the constructor with an initial value does its job.
        /// We also test if the `get` message works for account id.
        /// We also test if the `inc` message works for account id.
        #[ink::test]
        fn it_works() {
            let mut incrementer = Incrementer::new(42);
            assert_eq!(incrementer.get(), 42);
            incrementer.inc(5);
            assert_eq!(incrementer.get(), 47);
            incrementer.inc(-50);
            assert_eq!(incrementer.get(), -3);
        }

        /// We test if the constructor with an initial value does its job.
        /// We also test if the `get` message works for account id.
        /// We also test if the `inc` message works for account id.
        /// We also test if the `get_mine` message works for account id.
        #[ink::test]
        fn my_value_works() {
            let mut incrementer = Incrementer::new(11);
            assert_eq!(incrementer.get(), 11);
            assert_eq!(incrementer.get_mine(), 0);
            incrementer.inc_mine(5);
            assert_eq!(incrementer.get_mine(), 5);
            incrementer.inc_mine(10);
            assert_eq!(incrementer.get_mine(), 15);
        }
    }
}

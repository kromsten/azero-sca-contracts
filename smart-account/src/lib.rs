#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod contract;
mod error;

pub use contract::abstract_account::{
  AccountContract,
  AccountContractRef,
};
  
#[cfg(test)]
mod tests;
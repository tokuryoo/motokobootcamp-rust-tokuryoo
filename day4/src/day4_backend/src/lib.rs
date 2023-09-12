use ic_cdk::{export::candid::Principal, query, update};
mod types; // types.rs
use std::cell::RefCell;
use std::collections::HashMap;
use types::Account;
use types::Result;

thread_local! {
    static LEDGER: RefCell<HashMap<Account, u128>> = RefCell::new(HashMap::new());
}

#[query]
fn name() -> String {
    String::from("MotoCoin")
}

#[query]
fn symbol() -> String {
    String::from("MOC")
}

// 誰でもエアドロップし放題
#[update]
fn airdrop(account: Account) -> Result {
    ic_cdk::println!("airdrop");
    LEDGER.with(|ledger| {
        let mut ledger = ledger.borrow_mut();
        let balance = ledger.get_mut(&account);
        match balance {
            None => {
                ledger.insert(account, 10);
            }
            Some(balance) => {
                *balance += 10;
            }
        }
    });
    Result::ok
    // TODO Result::err("xxxx");
}

#[query(name = "balanceOf")]
fn balance_of(account: Account) -> u128 {
    ic_cdk::println!("balanceOf");
    LEDGER.with(|ledger| {
        let ledger = ledger.borrow();
        let balance = ledger.get(&account);
        match balance {
            None => 0,
            Some(balance) => *balance,
        }
    })
}

#[query(name = "totalSupply")]
fn total_supply() -> u128 {
    ic_cdk::println!("totalSupply");
    LEDGER.with(|ledger| {
        let ledger = ledger.borrow();
        let mut total = 0;
        for value in ledger.values() {
            total += value;
        }
        total
    })
}

#[update]
fn transfer(from: Account, to: Account, amount: u128) -> Result {
    let caller: Principal = ic_cdk::api::caller();
    if from.owner != caller {
        return Result::err("Not authorized.".to_string());
    }
    LEDGER.with(|ledger| {
        // TODO 入力チェック。残高が不足していないか等。
        let mut ledger = ledger.borrow_mut();
        let balance_from = ledger.get_mut(&from);
        match balance_from {
            None => (),
            Some(balance_from) => *balance_from -= amount,
        }
        let balance_to = ledger.get_mut(&to);
        match balance_to {
            None => (),
            Some(balance_to) => *balance_to += amount,
        }
        Result::ok
    })
}

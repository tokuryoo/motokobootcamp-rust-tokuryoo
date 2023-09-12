use ic_cdk::export::candid::CandidType;
use ic_cdk::export::candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType)]
pub enum Result {
    ok,
    err(String),
}

// https://internetcomputer.org/docs/current/references/candid-ref#type-blob
// type SubAccount = Vec<u8>;
// または
// type SubAccount = &[u8];

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash)]
pub struct Account {
    pub owner: Principal,
    // pub subaccount: Option<SubAccount>,
}

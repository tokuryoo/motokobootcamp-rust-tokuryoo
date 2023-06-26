use ic_cdk::export::candid::CandidType;
use ic_cdk::export::candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum Content {
    text(String),
}

#[derive(CandidType, Clone)]
pub struct Message {
    pub content: Content,
    pub creator: Principal,
    pub vote: i128,
}

#[derive(CandidType)]
pub enum Result1 {
    ok(Message),
    err(String),
}

// Rust 言語で提供されている Result は、Candid の Result に対応していないため、代わりに下記を定義。
// Rust の enum は Candid の variant に対応している様子。
// Serialize, Deserialize を書かなくても、エラーにはならない様子。
#[derive(CandidType)]
pub enum Result {
    ok,
    err(String),
}

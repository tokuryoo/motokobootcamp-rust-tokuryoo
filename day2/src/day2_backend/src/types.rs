use ic_cdk::export::candid::CandidType;
use serde::{Deserialize, Serialize};

// 例 CandidType
// https://github.com/dfinity/cdk-rs/blob/main/examples/chess/src/chess_rs/lib.rs

// #[derive(CandidType, Serialize)]
// コンパイルエラー。Deserialize も必要。
//     | #[update]
//     | ^^^^^^^^^ the trait `Deserialize<'_>` is not implemented for `Homework`
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Homework {
    pub completed: bool,
    pub description: String,
    pub title: String,
}

// Rust 言語で提供されている Result は、Candid の Result に対応していないため、代わりに下記を定義。
// Rust の enum は Candid の variant に対応している様子。
// Serialize, Deserialize を書かなくても、エラーにはならない様子。
#[derive(CandidType)]
pub enum UnitResult {
    ok,
    err(String),
}

#[derive(CandidType)]
pub enum HomeworkResult {
    ok(Homework),
    err(String),
}

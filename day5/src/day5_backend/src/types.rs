use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType)]
pub enum Result {
    ok,
    err(String),
}

#[derive(CandidType)]
pub enum Result1 {
    ok(StudentProfile),
    err(String),
}

#[derive(CandidType)]
pub enum TestResult {
    ok,
    err(TestError),
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct StudentProfile {
    pub name: String,
    pub team: String,
    pub graduate: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum TestError {
    UnexpectedError(String),
    UnexpectedValue(String),
}

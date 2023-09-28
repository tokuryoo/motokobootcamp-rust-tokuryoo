mod types;
use candid::types::principal::Principal;
use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;
use types::{Result, Result1, StudentProfile, TestError, TestResult};

thread_local! {
    static PROFILE_MAP: RefCell<HashMap<Principal, StudentProfile>> = RefCell::new(HashMap::new());
}

#[update(name = "addMyProfile")]
fn add_my_profile(profile: StudentProfile) -> Result {
    ic_cdk::println!("add_my_profile");
    let caller: Principal = ic_cdk::api::caller();
    ic_cdk::println!("caller: {caller}");
    PROFILE_MAP.with(|profile_map| {
        let mut profile_map = profile_map.borrow_mut();
        let existing_profile = profile_map.get(&caller);
        match existing_profile {
            None => {
                profile_map.insert(caller, profile);
                Result::ok
            }
            Some(_) => Result::err("It already exists.".to_string()),
        }
    })
}

#[update(name = "deleteMyProfile")]
fn delete_my_profile() -> Result {
    ic_cdk::println!("delete_my_profile");
    let caller: Principal = ic_cdk::api::caller();
    PROFILE_MAP.with(|profile_map| {
        let mut profile_map = profile_map.borrow_mut();
        let value = profile_map.remove(&caller);
        match value {
            None => Result::err("It does not exist.".to_string()),
            Some(_) => Result::ok,
        }
    })
}

#[update(name = "updateMyProfile")]
fn update_my_profile(profile: StudentProfile) -> Result {
    ic_cdk::println!("add_my_profile");
    let caller: Principal = ic_cdk::api::caller();
    PROFILE_MAP.with(|profile_map| {
        let mut profile_map = profile_map.borrow_mut();
        let existing_profile = profile_map.get(&caller);
        match existing_profile {
            None => Result::err("It does not exist.".to_string()),
            Some(_) => {
                profile_map.insert(caller, profile);
                Result::ok
            }
        }
    })
}

#[query(name = "seeAProfile")]
fn see_a_profile(princpal: Principal) -> Result1 {
    PROFILE_MAP.with(|profile_map| {
        let profie_map = profile_map.borrow();
        let profile = profie_map.get(&princpal);
        match profile {
            Some(profile) => Result1::ok(profile.clone()),
            None => Result1::err("It does not exist.".to_string()),
        }
    })
}

// 引数で day1 のキャニスターを指定
#[update(name = "test")]
async fn test(canister: Principal) -> TestResult {
    ic_cdk::println!("test");
    // https://forum.dfinity.org/t/inter-cansister-update-calls-in-rust/19623/2
    let _result_reset: CallResult<(f64,)> = call(canister, "reset", ()).await;
    let result: CallResult<(f64,)> = call(canister, "add", (1f64,)).await;
    match result {
        Ok(value) => {
            if value.0 == 1f64 {
                TestResult::ok
            } else {
                TestResult::err(TestError::UnexpectedValue(format!(
                    "Unexpected value: {:?}",
                    value.0
                )))
            }
        }
        Err(e) => TestResult::err(TestError::UnexpectedError(format!(
            "Unexpected error: {:?}",
            e
        ))),
    }
}

use ic_cdk::api::management_canister::main::{
    canister_info, CanisterInfoRequest, CanisterInfoResponse,
};

// 自身がコントローラーの場合のみ、canister_status を呼び出すことができるので、下記は却下。
// use ic_cdk::api::management_canister::main::{
//     canister_status, CanisterIdRecord, CanisterStatusResponse,
// };

// dfx identity get-principal
#[update(name = "verifyOwnership")]
async fn verify_ownership(canister: Principal, principal: Principal) -> bool {
    // https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-canister-info
    // https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/main/fn.canister_info.html
    let arg = CanisterInfoRequest {
        canister_id: canister,
        num_requested_changes: Some(10),
    };
    let info = canister_info(arg).await.unwrap();
    let controllers = info.0.controllers;
    ic_cdk::println!("controllers: {:?}", controllers);
    controllers.contains(&principal)
}

// cd ../day1
// dfx deploy
// ../day5
// dfx deploy
// dfx canister call day5_backend addMyProfile '(record {graduate=false; name="aaa"; team="bbb"})'
// dfx identity get-principal
// dfx canister call day5_backend verifyWork '(principal <day1のキャニスターのid>, principal "自分のprincipal")'
// dfx canister call day5_backend verifyWork '(principal "bw4dl-smaaa-aaaaa-qaacq-cai", principal "c6oov-ntjsg-mpt26-plmiq-h7dtr-6mpwx-g4qka-piapl-hc6gv-yhf77-qae")'
// dfx canister call day5_backend seeAProfile '(principal "c6oov-ntjsg-mpt26-plmiq-h7dtr-6mpwx-g4qka-piapl-hc6gv-yhf77-qae")'
#[update(name = "verifyWork")]
async fn verify_work(canister: Principal, principal: Principal) -> Result {
    let exist: bool = PROFILE_MAP.with(|profile_map| {
        let profie_map = profile_map.borrow();
        let profile = profie_map.get(&principal);
        profile.is_some()
    });
    if exist {
        let ownership = verify_ownership(canister, principal).await;
        if ownership {
            let test_result = test(canister).await;
            match test_result {
                TestResult::ok => {
                    PROFILE_MAP.with(|profile_map| {
                        let mut profile_map = profile_map.borrow_mut();
                        let existing_profile = profile_map.get_mut(&principal);
                        if let Some(profile) = existing_profile {
                            profile.graduate = true;
                        }
                    });
                    Result::ok
                }
                TestResult::err(e) => Result::err(format!("err1 {:?}", e)),
            }
        } else {
            Result::err("b".to_string())
        }
    } else {
        Result::err("c".to_string())
    }
}

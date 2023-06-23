// https://docs.rs/candid/latest/candid/types/number/struct.Nat.html
// use candid::Nat;
use ic_cdk::{query, update};
use std::cell::RefCell;
mod types; // types.rs
use types::{Homework, HomeworkResult, UnitResult};

// 例 CandidType
// https://github.com/dfinity/cdk-rs/blob/main/examples/chess/src/chess_rs/lib.rs

thread_local! {
    static HOMEWORKS: RefCell<Vec<Homework>> = RefCell::new(Vec::new());
    // RefCell はコンパイル時に借用チェックが行われず、違反していると、実行時にパニックになるので注意。
    // borrow_mut（可変参照） と borrow（不変参照）が同時に存在することは許されない。ライフタイムに注意。
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn getAllHomework() -> Vec<Homework> {
    // homeworks.borrow() の戻り値が Ref<Vec<Homework>> であるため clone
    HOMEWORKS.with(|homeworks| homeworks.borrow().clone())
}

#[query]
fn getPendingHomework() -> Vec<Homework> {
    // |arg| {} は無名関数。
    HOMEWORKS.with(|homeworks| {
        let hs = homeworks.borrow();
        let mut returned_hs: Vec<Homework> = Vec::new();
        // TODO 要件上 !x.completed で正しい？
        for h in hs.iter().filter(|x| !x.completed) {
            returned_hs.push(h.clone());
        }
        returned_hs
    })
}

#[query]
fn searchHomework(searchTerm: String) -> Vec<Homework> {
    HOMEWORKS.with(|homeworks| {
        let hs = homeworks.borrow();
        let mut returned_hs: Vec<Homework> = Vec::new();
        for h in hs.iter() {
            if h.title.contains(&searchTerm) || h.description.contains(&searchTerm) {
                returned_hs.push(h.clone());
            }
        }
        returned_hs
    })
}

#[query]
fn size() -> u128 {
    HOMEWORKS.with(|homeworks| {
        let hs = homeworks.borrow();
        hs.len() as u128
    })
}

#[query]
fn getHomework(index: u128) -> HomeworkResult {
    HOMEWORKS.with(|homeworks| {
        let hs = homeworks.borrow();
        let h = hs.get(index as usize);
        match h {
            None => HomeworkResult::err("index out of bounds".to_string()),
            Some(value) => HomeworkResult::ok(value.clone()),
        }
    })
}

// https://internetcomputer.org/docs/current/references/candid-ref#corresponding-rust-type-2
// Candid の nat 型に対応する Rust の型は candid::Nat または u128
#[update]
fn addHomework(homework: Homework) -> u128 {
    // with メソッドの戻り値の型は、クロージャの戻り値の型と同じになる。
    HOMEWORKS.with(|homeworks| {
        let mut hs = homeworks.borrow_mut();
        hs.push(homework);
        let index: usize = hs.len() - 1;
        // u128 > usize なのでエラーになる心配はない。
        index as u128 // クロージャの戻り値
    }) // セミコロン無しなので、addHomework がこの値を返すことになる。
}

#[update]
fn markAsCompleted(index: u128) -> UnitResult {
    HOMEWORKS.with(|homeworks| {
        let mut hs = homeworks.borrow_mut();
        let h = hs.get_mut(index as usize);
        match h {
            None => UnitResult::err("index out of bounds".to_string()),
            Some(x) => {
                x.completed = true;
                UnitResult::ok // セミコロン無し
            }
        }
    }) // セミコロン無し
}

#[update]
fn updateHomework(index: u128, homework: Homework) -> UnitResult {
    HOMEWORKS.with(|homeworks| {
        let mut hs = homeworks.borrow_mut();
        if hs.get(index as usize).is_some() {
            // hs の index を homework に差し替えたい。実装方法は？
            hs.remove(index as usize);
            hs.insert(index as usize, homework);
            UnitResult::ok // セミコロン無し
        } else {
            UnitResult::err("index out of bounds".to_string()) // セミコロン無し
        }
    }) // セミコロン無し
}

#[update]
fn deleteHomework(index: u128) -> UnitResult {
    HOMEWORKS.with(|homeworks| {
        let mut hs = homeworks.borrow_mut();
        if hs.get(index as usize).is_some() {
            hs.remove(index as usize);
            UnitResult::ok
        } else {
            UnitResult::err("index out of bounds".to_string())
        }
    }) // セミコロン無し
}

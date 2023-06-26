use ic_cdk::{export::candid::Principal, query, update};
use std::cell::RefCell;
mod types; // types.rs
use std::collections::HashMap;
use types::{Content, Message, Result, Result1};

thread_local! {
    static MESSAGE_ID: RefCell<u128> = RefCell::new(0);
    static MESSAGES: RefCell<HashMap<u128, Message>> = RefCell::new(HashMap::new());
    // RefCell はコンパイル時に借用チェックが行われず、違反していると、実行時にパニックになるので注意。
    // borrow_mut（可変参照） と borrow（不変参照）が同時に存在することは許されない。ライフタイムに注意。
}

#[update(name = "writeMessage")]
fn write_message(content: Content) -> u128 {
    let principal_id: Principal = ic_cdk::api::caller();
    let message = Message {
        content: content,
        creator: principal_id,
        vote: 0,
    };
    // * をつけることで、RefCell の中の値を取得できる。デリファレンス。
    let id = MESSAGE_ID.with(|id| *id.borrow());
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        messages.insert(id, message);
    });
    MESSAGE_ID.with(|id| {
        let mut id_mut = id.borrow_mut();
        *id_mut += 1;
    });
    id
}

#[query(name = "getMessage")]
fn get_message(message_id: u128) -> Result1 {
    MESSAGES.with(|messages| {
        let messages = messages.borrow();
        match messages.get(&message_id) {
            None => Result1::err("It does not exist.".to_string()),
            Some(message) => Result1::ok(message.clone()),
        }
    })
}

#[update(name = "deleteMessage")]
fn delete_message(message_id: u128) -> Result {
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        match messages.remove(&message_id) {
            None => Result::err("It does not exist.".to_string()),
            Some(_message) => Result::ok,
        }
    })
}

#[query(name = "getAllMessages")]
fn get_all_messages() -> Vec<Message> {
    // error[E0599]: no method named `values` found for reference `&RefCell<HashMap<u128, Message>>` in the current scope
    MESSAGES.with(|messages| messages.borrow().values().cloned().collect())
}

#[update(name = "upVote")]
fn up_vote(message_id: u128) -> Result {
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        let mut message = messages.get_mut(&message_id);
        match message {
            None => Result::err("It does not exist.".to_string()),
            Some(message) => {
                message.vote += 1;
                Result::ok
            }
        }
    })
}

#[update(name = "downVote")]
fn down_vote(message_id: u128) -> Result {
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        let mut message = messages.get_mut(&message_id);
        match message {
            None => Result::err("It does not exist.".to_string()),
            Some(message) => {
                message.vote -= 1;
                Result::ok
            }
        }
    })
}

#[query(name = "getAllMessagesRanked")]
fn get_all_messages_ranked() -> Vec<Message> {
    MESSAGES.with(|messages| {
        let mut messages: Vec<Message> = messages.borrow().values().cloned().collect();
        messages.sort_by(|m1, m2| m2.vote.cmp(&m1.vote));
        messages
    })
}

#[update(name = "updateMessage")]
fn update_message(message_id: u128, content: Content) -> Result {
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        match messages.get_mut(&message_id) {
            None => Result::err("It does not exist.".to_string()),
            Some(message) => {
                message.content = content;
                Result::ok
            }
        }
    })
}

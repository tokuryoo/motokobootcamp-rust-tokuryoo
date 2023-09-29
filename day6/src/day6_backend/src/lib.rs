use candid::types::principal::Principal;
use std::cell::RefCell;
use std::collections::HashMap;

// static mut value: f64 = 0f64;

thread_local! {
    static CALCULATED_VALUE_MAP: RefCell<HashMap<Principal, f64>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
fn add(x: f64) -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v + x;
                value_map.insert(caller, new_value);
                new_value
            }
            None => {
                value_map.insert(caller, x);
                x
            }
        }
    })
}

#[ic_cdk::update]
fn div(x: f64) -> Option<f64> {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        if x == 0f64 {
            return None;
        }
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v / x;
                value_map.insert(caller, new_value);
                Some(new_value)
            }
            None => Some(0f64),
        }
    })
}

#[ic_cdk::update]
fn floor() -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v.floor();
                value_map.insert(caller, new_value);
                new_value
            }
            None => 0f64,
        }
    })
}

#[ic_cdk::update]
fn mul(x: f64) -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v * x;
                value_map.insert(caller, new_value);
                new_value
            }
            None => 0f64,
        }
    })
}

#[ic_cdk::update]
fn power(x: f64) -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v.powf(x);
                value_map.insert(caller, new_value);
                new_value
            }
            None => 0f64,
        }
    })
}

#[ic_cdk::update]
fn reset() {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        value_map.remove(&caller);
    })
}

#[ic_cdk::query]
fn see() -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let value_map = value_map.borrow();
        let value = value_map.get(&caller);
        match value {
            Some(value) => *value,
            None => 0f64,
        }
    })
}

#[ic_cdk::update]
fn sqrt() -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v.sqrt();
                value_map.insert(caller, new_value);
                new_value
            }
            None => 0f64,
        }
    })
}

#[ic_cdk::update]
fn sub(x: f64) -> f64 {
    let caller: Principal = ic_cdk::api::caller();
    CALCULATED_VALUE_MAP.with(|value_map| {
        let mut value_map = value_map.borrow_mut();
        let value = value_map.get(&caller);
        match value {
            Some(v) => {
                let new_value = v - x;
                value_map.insert(caller, new_value);
                new_value
            }
            None => {
                value_map.insert(caller, x);
                x
            }
        }
    })
}

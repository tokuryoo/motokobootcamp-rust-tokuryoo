// 今回は、thread_local ではない。day 2 で thread_local を試す。
static mut value: f64 = 0f64;

#[ic_cdk::update]
fn add(x: f64) -> f64 {
    unsafe {
        value += x;
        value
    }
}

#[ic_cdk::update]
fn div(x: f64) -> Option<f64> {
    if x == 0f64 {
        return None;
    }
    unsafe {
        value /= x;
        Some(value)
    }
}

#[ic_cdk::update]
fn floor() -> f64 {
    unsafe {
        value = value.floor();
        value
    }
}

#[ic_cdk::update]
fn mul(x: f64) -> f64 {
    unsafe {
        value *= x;
        value
    }
}

#[ic_cdk::update]
fn power(x: f64) -> f64 {
    unsafe {
        value = value.powf(x);
        value
    }
}

#[ic_cdk::update]
fn reset() {
    unsafe {
        value = 0f64;
    }
}
#[ic_cdk::query]
fn see() -> f64 {
    unsafe { value }
}

#[ic_cdk::update]
fn sqrt() -> f64 {
    unsafe {
        value = value.sqrt();
        value
    }
}

#[ic_cdk::update]
fn sub(x: f64) -> f64 {
    unsafe {
        value -= x;
        value
    }
}

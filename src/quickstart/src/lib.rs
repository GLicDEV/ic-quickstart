mod env;
mod lifetime;

use crate::env::{CanisterEnv, EmptyEnv, Environment};
use candid::{candid_method, CandidType};
use ic_cdk::print;
use ic_cdk_macros::*;
use serde::Deserialize;

use std::cell::{Ref, RefCell};

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnv {}),
            data: Data::default(),
        }
    }
}

#[derive(CandidType, Default, Deserialize)]
struct Data {
    items: Vec<String>,
}

#[candid_method(query)]
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}! Welcome!", name)
}

#[candid_method(update)]
#[update(name = "add")]
fn add(name: String) -> bool {
    RUNTIME_STATE.with(|state| add_impl(name, &mut state.borrow_mut()))
}

fn add_impl(name: String, runtime_state: &mut RuntimeState) -> bool {
    let now = runtime_state.env.now();

    // id is printed just to check that random works as expected
    print(format!(
        "Adding {} at {} with id {}",
        name,
        now,
        runtime_state.env.random_u32()
    ));
    runtime_state.data.items.push(name);

    true
}

#[candid_method(query, rename = "getAll")]
#[query(name = "getAll")]
fn get_all() -> Vec<String> {
    RUNTIME_STATE.with(|state| get_all_impl(state.borrow()))
}

fn get_all_impl(runtime_state: Ref<RuntimeState>) -> Vec<String> {
    runtime_state.data.items.iter().cloned().collect()
}

// Auto export the candid interface
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

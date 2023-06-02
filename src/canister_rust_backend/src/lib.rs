use ic_cdk::export::candid;
use ic_cdk_macros::*;

#[import(canister = "multiply_deps")]
struct CounterCanister;

#[update]
async fn read() -> candid::Nat {
    CounterCanister::read().await.0
}

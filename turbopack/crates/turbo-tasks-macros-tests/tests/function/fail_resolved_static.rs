#![feature(arbitrary_self_types)]
#![feature(arbitrary_self_types_pointers)]
#![allow(dead_code)]

use turbo_tasks::{ResolvedVc, Vc};

#[turbo_tasks::value(transparent)]
struct IntegersVec(Vec<ResolvedVc<u32>>);

#[turbo_tasks::function(resolved)]
fn return_contains_unresolved_vc() -> Vc<IntegersVec> {
    Vc::cell(Vec::new())
}

fn main() {}

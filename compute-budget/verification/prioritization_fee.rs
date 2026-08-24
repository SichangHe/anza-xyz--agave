//! Verus specification and lemmas for the production prioritization-fee helper.
//!
//! The evaluator injects these proof items around the balanced-token extraction
//! of `get_prioritization_fee`; this file intentionally contains no executable
//! copy of that function.

use vstd::arithmetic::div_mod::{
    lemma_div_by_multiple, lemma_div_is_ordered, lemma_div_pos_is_pos, lemma_remainder_lower,
    lemma_remainder_upper,
};
use vstd::arithmetic::mul::lemma_mul_strictly_positive;
use vstd::prelude::*;

verus! {

pub open spec fn prioritization_fee_product(price: u64, limit: u64) -> int {
    price as int * limit as int
}

pub open spec fn prioritization_fee_ceiling(price: u64, limit: u64) -> int {
    (prioritization_fee_product(price, limit) + 999_999int) / 1_000_000int
}

pub open spec fn prioritization_fee_spec(price: u64, limit: u64) -> int {
    if prioritization_fee_ceiling(price, limit) > u64::MAX as int {
        u64::MAX as int
    } else {
        prioritization_fee_ceiling(price, limit)
    }
}

pub proof fn lemma_product_and_bias_fit_u128(price: u64, limit: u64)
    ensures
        prioritization_fee_product(price, limit) + 999_999 <= u128::MAX as int,
{
    assert(u64::MAX as int * u64::MAX as int + 999_999 <= u128::MAX as int) by (compute);
    assert(prioritization_fee_product(price, limit) + 999_999 <= u128::MAX as int) by (nonlinear_arith);
}

pub proof fn lemma_prioritization_fee_properties(price: u64, limit: u64)
    ensures
        0 <= prioritization_fee_spec(price, limit) <= u64::MAX as int,
        prioritization_fee_spec(price, limit) == 0 <==> price == 0 || limit == 0,
        prioritization_fee_product(price, limit) > 0
            && prioritization_fee_ceiling(price, limit) <= u64::MAX as int ==> {
                let fee = prioritization_fee_spec(price, limit);
                &&& (fee - 1) * 1_000_000 < prioritization_fee_product(price, limit)
                &&& prioritization_fee_product(price, limit) <= fee * 1_000_000
            },
        prioritization_fee_ceiling(price, limit) > u64::MAX as int ==>
            prioritization_fee_spec(price, limit) == u64::MAX as int,
{
    let product = prioritization_fee_product(price, limit);
    let biased = product + 999_999;
    let ceiling = prioritization_fee_ceiling(price, limit);

    assert(product == price as int * limit as int);
    assert(biased == product + 999_999);
    assert(ceiling == biased / 1_000_000);
    lemma_div_pos_is_pos(biased, 1_000_000);
    lemma_remainder_lower(biased, 1_000_000);
    lemma_remainder_upper(biased, 1_000_000);

    if price == 0 {
        assert(price as int == 0);
        assert(product == 0);
        assert(biased == 999_999);
        assert(999_999int / 1_000_000int == 0) by (compute);
    } else if limit == 0 {
        assert(limit as int == 0);
        assert(product == 0);
        assert(biased == 999_999);
        assert(999_999int / 1_000_000int == 0) by (compute);
    } else {
        assert(price >= 1 && limit >= 1);
        assert(price as int >= 1 && limit as int >= 1);
        lemma_mul_strictly_positive(price as int, limit as int);
        assert(product >= 1);
        assert(biased >= 1_000_000);
        lemma_div_is_ordered(1_000_000, biased, 1_000_000);
        lemma_div_by_multiple(1, 1_000_000);
        assert(ceiling >= 1);
    }
}

}

fn main() {}

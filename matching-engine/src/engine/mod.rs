pub mod engine_impl;
pub use engine_impl::MatchingEngine;

mod order;
pub use order::Order;

mod order_result;
pub use {
    order_result::OrderResult,
    order_result::OrderType,
    order_result::OrderResultType
};

mod order_match_result;
pub use order_match_result::OrderMatchResult;

pub mod tests;

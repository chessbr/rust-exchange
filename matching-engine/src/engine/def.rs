use super::order_result::OrderResult;
use super::order::Order;

pub struct MatchingEngine {
    asset: String,
}

impl MatchingEngine {
    pub fn new(asset: String) -> MatchingEngine {
        MatchingEngine {
            asset,
        }
    }
    pub fn add_order(&self, order: &Order) -> Vec<OrderResult> {
        let results: Vec<OrderResult> = Vec::new();
        return results;
    }
}

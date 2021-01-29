use super::Order;

#[derive(Debug)]
pub struct OrderMatchResult {
    pub remaining_order: Option<Order>,
    pub executed_order: Option<Order>,
}

impl OrderMatchResult {
    pub fn new(executed_order: Option<Order>, remaining_order: Option<Order>) -> OrderMatchResult {
        OrderMatchResult {
            remaining_order,
            executed_order,
        }
    }
}

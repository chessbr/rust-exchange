#[derive(Debug, PartialEq, Eq)]
pub enum OrderResultType {
    QUEUED,
    EXECUTED
}
#[derive(Debug, PartialEq, Eq)]
pub enum OrderType {
    BUY,
    SELL
}
#[derive(Debug)]
pub struct OrderResult {
    pub result_type: OrderResultType,
    pub order_type: OrderType,
    pub quantity: u64,
    pub price: f32,
}

impl OrderResult {
    pub fn new(result_type: OrderResultType, order_type: OrderType, quantity: u64, price: f32) -> OrderResult {
        OrderResult {
            result_type,
            order_type,
            quantity,
            price,
        }
    }
}

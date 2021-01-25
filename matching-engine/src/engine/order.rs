use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Order {
    pub order_type: super::OrderType,
    pub quantity: u64,
    pub price: f32,
    pub instant: Instant,
}
impl Order {
    pub fn new(order_type: super::OrderType, quantity: u64, price: f32, instant: Instant) -> Order {
        Order {
            order_type,
            quantity,
            price,
            instant,
        }
    }
}

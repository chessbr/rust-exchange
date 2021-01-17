pub struct Order {
    pub order_type: super::OrderType,
    pub quantity: u64,
    pub price: f32,
}
impl Order {
    pub fn new(order_type: super::OrderType, quantity: u64, price: f32) -> Order {
        Order {
            order_type,
            quantity,
            price,
        }
    }
}

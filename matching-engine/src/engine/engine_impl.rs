use std::time::Instant;
use super::order_result::OrderResult;
use super::order::Order;
use super::OrderMatchResult;

#[derive(Debug)]
pub struct MatchingEngine {
    asset: String,
    buy_order_book: Vec<Order>,
    sell_order_book: Vec<Order>,
}

impl MatchingEngine {
    pub fn new(asset: String) -> MatchingEngine {
        MatchingEngine {
            asset,
            buy_order_book: Vec::new(),
            sell_order_book: Vec::new(),
        }
    }

    pub fn get_asset(&self) -> &String {
        return &self.asset;
    }

    pub fn get_buy_order_book(&self) -> &Vec<Order> {
        return &self.buy_order_book;
    }

    pub fn get_sell_order_book(&self) -> &Vec<Order> {
        return &self.sell_order_book;
    }

    /// Find the index where the given `order` should be placed
    /// inside the given `order_book` according to the priorities
    fn find_order_place(order_book: &Vec<Order>, order: &Order) -> Option<usize> {
        // no orders, so this should be the first one
        if order_book.len() == 0 {
            return Some(0);
        }

        let first_order = &order_book[0];
        // order type doesn't match the order from the book, there's nothing to do here
        if first_order.order_type != order.order_type {
            return None;
        }

        for (index, order_from_book) in order_book.iter().enumerate() {
            // the given order is cheaper, it comes first
            if order.price < order_from_book.price {
                return Some(index);
            // the given order has the same price, but it was received
            // before the order from the book, it should come first
            } else if order.price == order_from_book.price && order.instant < order_from_book.instant {
                return Some(index);
            }
        }

        // Last position
        return Some(order_book.len());
    }

    fn enqueue_order(order_book: &mut Vec<Order>, order: &Order, results: &mut Vec<OrderResult>) {
        let result = OrderResult::new(
            super::OrderResultType::QUEUED,
            order.order_type.clone(),
            order.quantity,
            order.price,
        );
        results.push(result);

        // add the order to the book as we don't have any others to match
        let order_place = MatchingEngine::find_order_place(order_book, order);

        if order_place.is_some() {
            order_book.insert(order_place.unwrap(), order.clone());
        }
    }

    fn match_order(booked_order: &Order, received_order: &Order) -> OrderMatchResult {
        let executed_price;

        if booked_order.order_type == super::OrderType::BUY && received_order.order_type == super::OrderType::SELL && booked_order.price >= received_order.price {
            executed_price = booked_order.price;
        } else if booked_order.order_type == super::OrderType::SELL && received_order.order_type == super::OrderType::BUY && booked_order.price <= received_order.price {
            executed_price = booked_order.price;
        } else {
            // not executed
            return OrderMatchResult::new(Option::None, Option::None);
        }

        let executed_qty = std::cmp::min(received_order.quantity, booked_order.quantity);
        let remaining_qty = received_order.quantity - executed_qty;
        let mut remaining_order: Option<Order> = Option::None;

        if remaining_qty > 0 {
            // partially executed
            remaining_order = Some(
                Order::new(
                    received_order.order_type.clone(),
                    remaining_qty,
                    received_order.price,
                    received_order.instant.clone(),
                )
            );
        }

        return OrderMatchResult::new(
            Some(
                Order::new(
                    received_order.order_type.clone(),
                    executed_qty,
                    executed_price,
                    received_order.instant.clone(),
                )
            ),
            remaining_order,
        );
    }

    pub fn add_order(&mut self, order_type: super::OrderType, quantity: u64, price: f32) -> Vec<OrderResult> {
        let order = super::Order::new(order_type, quantity, price, Instant::now());

        // the list of match results
        let mut results: Vec<OrderResult> = Vec::new();
        // the book that will store the order
        let order_book: &mut Vec<Order>;
        // the book that the engine will try to match the order against
        let match_order_book: &mut Vec<Order>;

        if order.order_type == super::OrderType::BUY {
            order_book = &mut self.buy_order_book;
            match_order_book = &mut self.sell_order_book;
        } else { // order.order_type == super::OrderType::SELL
            order_book = &mut self.sell_order_book;
            match_order_book = &mut self.buy_order_book;
        }

        // there is no order to match, then just enqueue the order into the book
        if match_order_book.len() == 0 {
            MatchingEngine::enqueue_order(order_book, &order, &mut results);
            return results;
        }

        // has one or more orders, let's go through the orders and match them
        let mut current_order: Option<Order> = Some(order);
        while current_order.is_some() && match_order_book.len() > 0 {
            let current_order_value: &Order = current_order.as_ref().unwrap();

            // match the order with the first order of the book
            let match_result: super::OrderMatchResult = MatchingEngine::match_order(&match_order_book[0], &current_order_value);

            // the order didn't match, so enqueue it into the order book
            if match_result.executed_order.is_none() {
                MatchingEngine::enqueue_order(order_book, current_order_value, &mut results);
                return results;
            } else {
                let executed_order: &Order = match_result.executed_order.as_ref().unwrap();

                // assuming the vector is in priority order,
                // remove the first order from the vec
                let original_order_from_book = match_order_book.remove(0);

                // if there is no remaining order, it means it was fully executed
                // check if the original order dequeued from the book was fully executed
                // by checking the executed quantity
                if match_result.remaining_order.is_none() && executed_order.quantity < original_order_from_book.quantity {
                    // the order dequeued from the book was partially executed
                    // we should put the remaining order back to its original position
                    match_order_book.insert(0,
                        Order::new(
                            original_order_from_book.order_type.clone(),
                            original_order_from_book.quantity - executed_order.quantity,
                            original_order_from_book.price,
                            original_order_from_book.instant,
                        )
                    );
                }

                // the current order is the remaining order now
                current_order = match_result.remaining_order;

                let result = OrderResult::new(
                    super::OrderResultType::EXECUTED,
                    executed_order.order_type.clone(),
                    executed_order.quantity,
                    executed_order.price,
                );
                results.push(result);
            }
        }

        // save the remaining order to the book
        if current_order.is_some() {
            MatchingEngine::enqueue_order(order_book, current_order.as_ref().unwrap(), &mut results);
        }

        return results;
    }
}

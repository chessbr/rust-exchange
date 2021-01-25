#[test]
fn test_simple_order() {
    let engine = super::MatchingEngine::new(String::from("QWERTY"));
    let buy_order = super::Order::new(super::OrderType::BUY, 100, 9.90);
    let sell_order = super::Order::new(super::OrderType::SELL, 100, 9.90);

    let buy_order_results: Vec<super::OrderResult> = engine.add_order(&buy_order);
    assert_eq!(buy_order_results.len(), 1);
    let buy_result: &super::OrderResult = &buy_order_results[0];
    assert_eq!(buy_result.result_type, super::OrderResultType::QUEUED);
    assert_eq!(buy_result.order_type, super::OrderType::BUY);
    assert_eq!(buy_result.quantity, 100);
    assert_eq!(buy_result.price, 9.90);

    let sell_order_results: Vec<super::OrderResult> = engine.add_order(&sell_order);
    assert_eq!(sell_order_results.len(), 1);
    let sell_result: &super::OrderResult = &sell_order_results[0];
    assert_eq!(sell_result.result_type, super::OrderResultType::EXECUTED);
    assert_eq!(sell_result.order_type, super::OrderType::SELL);
    assert_eq!(sell_result.quantity, 100);
    assert_eq!(sell_result.price, 9.90);
}

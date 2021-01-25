#[test]
fn test_simple_order() {
    let mut engine = super::MatchingEngine::new(String::from("QWERTY"));
    assert_eq!(engine.get_asset().clone(), String::from("QWERTY"));

    let buy_order_results: Vec<super::OrderResult> = engine.add_order(super::OrderType::BUY,  100,  9.90);
    assert_eq!(buy_order_results.len(), 1);
    let buy_result: &super::OrderResult = &buy_order_results[0];
    assert_eq!(buy_result.result_type, super::OrderResultType::QUEUED);
    assert_eq!(buy_result.order_type, super::OrderType::BUY);
    assert_eq!(buy_result.quantity, 100);
    assert_eq!(buy_result.price, 9.90);

    let sell_order_results: Vec<super::OrderResult> = engine.add_order(super::OrderType::SELL, 100, 9.90);
    assert_eq!(sell_order_results.len(), 1);
    let sell_result: &super::OrderResult = &sell_order_results[0];
    assert_eq!(sell_result.result_type, super::OrderResultType::EXECUTED);
    assert_eq!(sell_result.order_type, super::OrderType::SELL);
    assert_eq!(sell_result.quantity, 100);
    assert_eq!(sell_result.price, 9.90);
}

#[test]
/// Test that items with are sorted by price
fn test_cheapest_price() {
    let mut engine = super::MatchingEngine::new(String::from("QWERTY"));

    // sell 1000 units at 50
    engine.add_order(super::OrderType::SELL, 1000, 50.0);
    // buy 100 units at 50
    engine.add_order(super::OrderType::BUY, 100, 51.0);
    // sell 200 units at 49
    engine.add_order(super::OrderType::SELL, 200, 49.0);

    // first order from the book is the cheapest price
    assert_eq!(engine.get_sell_order_book()[0].quantity, 200);
    assert_eq!(engine.get_sell_order_book()[0].price, 49.0);
    assert_eq!(engine.get_sell_order_book()[1].quantity, 900);
    assert_eq!(engine.get_sell_order_book()[1].price, 50.0);

    // buy 200 units at 49 and 100 units at 50
    let buy_order_results: Vec<super::OrderResult> = engine.add_order(super::OrderType::BUY, 300, 52.0);

    assert_eq!(buy_order_results.len(), 2);
    let first_execution: &super::OrderResult = &buy_order_results[0];
    assert_eq!(first_execution.result_type, super::OrderResultType::EXECUTED);
    assert_eq!(first_execution.order_type, super::OrderType::BUY);
    assert_eq!(first_execution.quantity, 200);
    assert_eq!(first_execution.price, 49.0);

    let second_execution: &super::OrderResult = &buy_order_results[1];
    assert_eq!(second_execution.result_type, super::OrderResultType::EXECUTED);
    assert_eq!(second_execution.order_type, super::OrderType::BUY);
    assert_eq!(second_execution.quantity, 100);
    assert_eq!(second_execution.price, 50.0);

    assert_eq!(engine.get_sell_order_book().len(), 1);
    assert_eq!(engine.get_sell_order_book()[0].quantity, 800);
    assert_eq!(engine.get_sell_order_book()[0].price, 50.0);
}

#[test]
/// Test that items with same price are ordered according to its received time
fn test_first_comes_first() {
    let mut engine = super::MatchingEngine::new(String::from("QWERTY"));

    // sell 1000 units at 50
    engine.add_order(super::OrderType::SELL, 1000, 50.0);
    // buy 100 units at 50
    engine.add_order(super::OrderType::BUY, 100, 51.0);
    // sell 100 units at 50
    engine.add_order(super::OrderType::SELL, 200, 50.0);

    // first order from the book is the one that arrived first
    assert_eq!(engine.get_sell_order_book()[0].quantity, 900);
    assert_eq!(engine.get_sell_order_book()[1].quantity, 200);

    // buy 200 units at 50
    let buy_order_results: Vec<super::OrderResult> = engine.add_order(super::OrderType::BUY, 200, 52.0);

    assert_eq!(buy_order_results.len(), 1);
    let first_execution: &super::OrderResult = &buy_order_results[0];
    assert_eq!(first_execution.result_type, super::OrderResultType::EXECUTED);
    assert_eq!(first_execution.order_type, super::OrderType::BUY);
    assert_eq!(first_execution.quantity, 200);
    assert_eq!(first_execution.price, 50.0);

    assert_eq!(engine.get_sell_order_book()[0].quantity, 700);
    assert_eq!(engine.get_sell_order_book()[1].quantity, 200);
}

#[test]
/// Test that items with same price are ordered according to its received time
fn test_partial_execution() {
    let mut engine = super::MatchingEngine::new(String::from("QWERTY"));

    // buy 100 units at 51
    engine.add_order(super::OrderType::BUY, 100, 51.0);

    // sell 1000 units at 50
    engine.add_order(super::OrderType::SELL, 1000, 50.0);

    assert_eq!(engine.get_sell_order_book()[0].quantity, 900);
    assert_eq!(engine.get_sell_order_book().len(), 1);
    assert_eq!(engine.get_buy_order_book().len(), 0);
}

#[test]
/// Test that items with same price are ordered according to its received time
fn test_not_executed() {
    let mut engine = super::MatchingEngine::new(String::from("QWERTY"));

    // buy 100 units at 50
    engine.add_order(super::OrderType::BUY, 100, 50.0);

    // sell 1000 units at 50.01
    engine.add_order(super::OrderType::SELL, 1000, 50.01);

    assert_eq!(engine.get_sell_order_book().len(), 1);
    assert_eq!(engine.get_buy_order_book().len(), 1);
    assert_eq!(engine.get_sell_order_book()[0].quantity, 1000);
    assert_eq!(engine.get_buy_order_book()[0].quantity, 100);
}

use std::env;
use tonic::{transport::Server, Request, Response, Status};
use std::sync::{Arc, Mutex};

mod engine;
use engine::{MatchingEngine, OrderType, OrderResult};

use order_protobuf::order_service_server::{OrderService, OrderServiceServer};
use order_protobuf::{OrderRequest, OrderResponse};

pub mod order_protobuf {
    tonic::include_proto!("order_protobuf");
}

#[derive(Debug)]
pub struct MatchingEngineOrderService {
    matching_engine: Arc<Mutex<MatchingEngine>>
}

#[tonic::async_trait]
impl OrderService for MatchingEngineOrderService {
    async fn send_order(
        &self,
        request: Request<OrderRequest>,
    ) -> Result<Response<OrderResponse>, Status> {

        let order_info = request.into_inner();
        let asset_code: String = String::from(order_info.asset_code);
        let order_type: i32 = order_info.order_type;
        let qty: u64 = order_info.quantity;
        let price: f32 = order_info.price;

        let mut order_type_enum: OrderType = OrderType::BUY;

        if order_type == 1 {
            order_type_enum = OrderType::SELL;
        } else if order_type != 0 {
            let response = order_protobuf::OrderResponse {
                ok: false,
                error: String::from("Invalid order type."),
            };
            return Ok(Response::new(response));
        }

        let mut engine = self.matching_engine.lock().unwrap();
        let results: Vec<OrderResult> = engine.add_order(order_type_enum, qty, price);
        for result in results {
            println!("Order {:?} {:?} => {:?} x {:?}", result.result_type, result.order_type, result.quantity, result.price);
        }

        let response = order_protobuf::OrderResponse {
            ok: true,
            error: String::from(""),
        };

        return Ok(Response::new(response));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let addr = "[::1]:3050".parse().unwrap();
        let asset: &String = &args[1];

        println!("\n\n*** Starting matching engine for {} ***\n\n", asset);

        let mut engine = MatchingEngine::new(asset.to_string());
        let mutex_engine = Arc::new(Mutex::new(engine));

        let order_service = MatchingEngineOrderService {
            matching_engine: mutex_engine,
        };

        Server::builder()
            .add_service(OrderServiceServer::new(order_service))
            .serve(addr)
            .await?;

    } else {
        println!("You need to inform the asset code to accept orders.");
    }

    Ok(())
}

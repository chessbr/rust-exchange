use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let asset: &String = &args[1];

    println!("Starting matching engine for {}", asset);

    loop {
        println!("Enter your order command in the format: BUY|SELL QTY PRICE");

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read you order command");
        let args: Vec<&str> = command.split(" ").collect();
        let args_size = args.len();

        if args_size != 3 {
            if args_size > 0 && args[0].trim().to_lowercase() == "q" {
                println!("Bye!");
                break;
            }

            println!("Wrong number of arguments in your command.");
            continue;
        }

        let order_type: String = args[0].trim().to_uppercase();
        let qty: u32 = args[1].trim().parse().expect("Invalid quantity");
        let price: f32 = args[2].trim().parse().expect("Invalid price");

        if order_type != "BUY" && order_type != "SELL" {
            println!("Invalid order type: {:?}", order_type);
            continue;
        }

        println!("Received an order for asset {:?} {:?} units for {:?}", order_type, qty, price);
    }
}

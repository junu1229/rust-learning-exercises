// Order Type enum with Buy/Sell variants
#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

// Order struct with id, order_type, amount, price fields
#[derive(Debug, Clone)]
pub struct Order {
    pub id: u32,
    pub order_type: OrderType,
    pub amount: f64,
    pub price: f64,
}

impl Order {
    pub fn new(id: u32, order_type: OrderType, amount: f64, price: f64) -> Self {
        Order {
            id,
            order_type,
            amount,
            price,
        }
    }
}

// OrderBook struct with buy_orders, sell_orders vectors, next_id
#[derive(Debug)]
pub struct OrderBook {
    pub buy_orders: Vec<Order>,
    pub sell_orders: Vec<Order>,
    pub next_id: u32,
}

impl OrderBook {
    // Initialize empty vectors, next_id = 1
    pub fn new() -> Self {
        OrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            next_id: 1,
        }
    }

    // Create order, increment ID, add to appropriate vector
    pub fn add_order(&mut self, order_type: OrderType, amount: f64, price: f64) {
        let order = Order::new(self.next_id, order_type.clone(), amount, price);
        self.next_id += 1;

        match order_type {
            OrderType::Buy => self.buy_orders.push(order),
            OrderType::Sell => self.sell_orders.push(order),
        }
    }

    // Display buy/sell orders separately
    pub fn show_order_book(&self) {
        println!("=== Order Book ===");

        println!("\nBuy Orders:");
        if self.buy_orders.is_empty() {
            println!("  No buy orders available.");
        } else {
            println!("  ID    | Amount   | Price");
            println!("  ------|----------|----------");
            for order in &self.buy_orders {
                println!(
                    "  {:4}  | {:8.2} | {:8.2}",
                    order.id, order.amount, order.price
                );
            }
        }

        println!("\nSell Orders:");
        if self.sell_orders.is_empty() {
            println!("  No sell orders available.");
        } else {
            println!("  ID    | Amount   | Price");
            println!("  ------|----------|----------");
            for order in &self.sell_orders {
                println!(
                    "  {:4}  | {:8.2} | {:8.2}",
                    order.id, order.amount, order.price
                );
            }
        }
        println!();
    }
}

fn main() {
    // Test with sample orders
    let mut order_book = OrderBook::new();

    println!("Order Book Exercise Test Started");

    // Add sample buy orders
    order_book.add_order(OrderType::Buy, 100.0, 50.0);
    order_book.add_order(OrderType::Buy, 200.0, 49.5);
    order_book.add_order(OrderType::Buy, 150.0, 51.0);

    // Add sample sell orders
    order_book.add_order(OrderType::Sell, 80.0, 52.0);
    order_book.add_order(OrderType::Sell, 120.0, 53.5);
    order_book.add_order(OrderType::Sell, 90.0, 51.5);

    // Display the order book
    order_book.show_order_book();

    // Show next ID
    println!("Next Order ID: {}", order_book.next_id);
}

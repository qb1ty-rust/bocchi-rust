#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
enum OrderStatus {
    Paid
}

struct Order {
    customer: String,
    status: OrderStatus,
    order: String,
}

struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f32,
    current_gas: f32,
    gas_consumption: f32,
}

impl Car {
    fn new(
        brand: &str,
        max_speed: u16,
        max_gas: f32,
        current_gas: f32,
        gas_consumption: f32,
    ) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            current_gas,
            gas_consumption,
        }
    }

    fn drive(&mut self, mut distance: f32) {
        let total_gas_consumed = distance * self.gas_per_km();

        loop {
            if self.current_gas <= 0.0 {
                println!("\nGas not enough");
                break;
            }

            if distance <= 0.0 {
                println!("\nDistance end");
                break;
            }

            self.current_gas = self.current_gas - self.gas_per_km();
            distance = distance - 1.0;
        }

        if total_gas_consumed > self.current_gas {
            println!(
                "Your car {} is actually not enough gas. Left {}",
                self.brand, self.current_gas
            );
        } else {
            println!(
                "Drive, you fucking idiot. Your max speed {}, and your max gas {}",
                self.max_speed, self.max_gas
            );
        }
    }

    fn gas_per_km(&self) -> f32 {
        self.gas_consumption / 100.0
    }

    fn order(&self, customer: &str) {
        let order = Order {
            customer: String::from(customer),
            status: OrderStatus::Paid,
            order: String::from(&self.brand),
        };

        println!(
            "\nCustomer is {}, his is order price status {:?}. His is order {}",
            order.customer, order.status, order.order
        )
    }
}

fn main() {
    let mut mercedes = Car::new("Mercedes", 320, 55.0, 50.0, 100.0);
    let mut volvo = Car::new("Volvo", 220, 80.0, 80.0, 10.0);

    let distance: f32 = 50.0;

    mercedes.drive(distance);
    volvo.drive(distance);

    let customer = "qb1ty";

    mercedes.order(customer);

    option_practice();
}

fn option_practice() {
    let value: Option<u8> = Some(100);

    if let Some(res) = value {
        println!("Sum: {}", res + 10);
    }

    match value {
        Some(res) => println!("Sum: {}", res + 10),
        None => (),
    }
}

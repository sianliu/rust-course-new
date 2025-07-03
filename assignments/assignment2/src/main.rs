// Define PaymentType enum with two variants
enum PaymentType {
    DigitalToken,
    Cash,
}

// Define Seller struct with payment_type, price, and balance fields
struct Seller {
    payment_type: PaymentType,
    price: f32, 
    balance: f32,
}

// DEfine Buyer struct with name, payment_type, and balance fields
struct Buyer {
    name: String,
    payment_type: PaymentType, 
    balance: f32, 
}

// Define BuyerGroup struct containing a vector of Buyer structs
struct BuyerGroup {
    members: Vec<Buyer>,
}

// 5-7. implement BuyerGroup block with 3 methods
impl BuyerGroup {
    // creates new empty BuyerGroup
    fn new() -> Self {
        BuyerGroup {
            members: Vec::new(),
        }
    }

    // Adds a buyer to the members vector
    fn add_member(&mut self, buyer: Buyer) {
        self.members.push(buyer);
    }

    // find buyer with matching payment_type, return index or -1 if not found
    fn find_buyer(&self, payment_type: PaymentType) -> i32 {
        for index in 0..self.members.len() {
            match payment_type {
                PaymentType::DigitalToken => {
                    match self.members[index].payment_type {
                        PaymentType::DigitalToken => return index as i32,
                        PaymentType::Cash => {},
                    }
                },
                PaymentType::Cash => {
                    match self.members[index].payment_type {
                        PaymentType::Cash => return index as i32,
                        PaymentType::DigitalToken => {},
                    }
                }
            }
        }
        -1 // return -1 if no matching buyer found
    }
    // Transfer money from buyer to seller repeatedly until buyer has insufficient balance
    fn buy(&mut self, buyer_index: usize, seller: &mut Seller) {
        if buyer_index >= self.members.len() {
            println!("Invalid buyer index");
            return;
        }

        let buyer = &mut self.members[buyer_index];

        // Check if payment types match using nested match statements
        let types_match = match buyer.payment_type {
            PaymentType::DigitalToken => {
                match seller.payment_type {
                    PaymentType::DigitalToken => true,
                    PaymentType::Cash => false,
                }
            },
            PaymentType::Cash => {
                match seller.payment_type {
                    PaymentType::Cash => true,
                    PaymentType::DigitalToken => false,
                }
            }
        };

        if !types_match {
            println!("Payment type mismatch!");
            return;
        }
        
        println!("Starting transaction: {} buying from seller", buyer.name);

        // Keep buying until buyer has insufficient balance
        let mut transactions = 0;
        while buyer.balance >= seller.price {
            buyer.balance -= seller.price;
            seller.balance += seller.price;
            transactions += 1;

            println!("Transaction {}: {} paid {}, remaining balance: {}", transactions, buyer.name, seller.price, buyer.balance);
        }

        if transactions == 0 {
            println!("{} has insufficient funds to make any purchases", buyer.name);
        } else {
            println!("Transaction complete. {} made {} purchases", buyer.name, transactions);
        }
    }
}

fn main() {
    // Create 2 buyers
    let sian = Buyer {
        name: String::from("Sian"), 
        payment_type: PaymentType::DigitalToken,
        balance: 100.0,
    };

    let jonathan = Buyer {
        name: String::from("Jonathan"), 
        payment_type: PaymentType::Cash, 
        balance: 100.0,
    };

    // Create an empty BuyerGroup
    let mut buyer_group = BuyerGroup::new();

    // Add buyers to the group sequentially
    buyer_group.add_member(sian);
    buyer_group.add_member(jonathan);

    // Create a seller
    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };

    // Find Jonathan's index (he uses cash payment type)
    let jonathan_index = buyer_group.find_buyer(PaymentType::Cash);

    if jonathan_index != -1 {
        println!("Found Jonathan at index: {}", jonathan_index);

        // Call buy method with Jonathan's index and the seller
        buyer_group.buy(jonathan_index as usize, &mut seller);
    } else {
        println!("Jonathan not found!");
    }

    // Display final state
    println!("\nFinal state:");
    for i in 0..buyer_group.members.len() {
        println!("Buyer {}: {} - Balance: {}", i, buyer_group.members[i].name, buyer_group.members[i].balance);
    }
    println!("Seller - Balance: {}", seller.balance);
}



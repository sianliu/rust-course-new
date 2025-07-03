// Define user struct with name and balance fields
struct User {
    name: String,
    balance: (f32, String),
}

// Define user method 
impl User {
    fn print_user_detail(&self) {
        println!("Name: {}, Balance: {} {}", self.name, self.balance.0, self.balance.1);
    }
}

// Define acrue interest function
fn accrue_interest(user: &mut User, interest_percentage: f32) {
    // calculate new balance with interest
    user.balance.0 = user.balance.0 + (user.balance.0 * interest_percentage / 100.0);

    // print updated user details
    user.print_user_detail();
}

fn main() {
    // Create a user variable with initial values
    let mut user = User {
        name: "Sian".to_owned(),
        balance: (100.0, "SGD".to_owned()),
    };

    // Print initial user details
    println!("Initial user details:");
    user.print_user_detail();
    println!();

    // Call accrue interest function with 5% interest
    println!("After applying 5% interest:");
    accrue_interest(&mut user, 5.0);
    println!();

    // Bonus: apply compounding interest multiple times
    println!("Applying compounding interest 3 times with 5% each time:");
    accrue_interest(&mut user, 5.0);
    accrue_interest(&mut user, 5.0);
    accrue_interest(&mut user, 5.0);
    println!();

    // Final balance
    println!("Final user details:");
    user.print_user_detail();
}


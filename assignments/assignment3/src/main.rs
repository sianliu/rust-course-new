// define struct of UserAccount with field: name (String) and age (Option<u32>)
struct UserAccount {
    name: String, 
    age: Option<u32>,
}

// define a trait called Balance and function to get balance 
trait Balance {
    fn get_balance(&self) -> u32 {
        10
    }
}

// implement trait Balance to UserAccount struct
impl Balance for UserAccount {}

// create function increase_balance which takes as arguments 
// - type that implements Balance trait
// - u32 amount parameter containing amount to increase
// - if increase amount is <= 10 return OK and increase balance by amount
// - if increase amount is > 10, return Err with increase must be less than 10
// return Result<u32, String>
fn increase_balance<T: Balance>(account: &T, amount: u32) -> Result<u32, String> {
    if amount <= 10 {
        Ok(account.get_balance() + amount)
    } else {
        Err("Increase must be less than 10!".to_string())
    }
}

fn main() {
    // create user_account and set age as Option::None
    let user_account = UserAccount {
        name: "Jonathan".to_string(),
        age: Option::None,
    };

    match increase_balance(&user_account, 11) {
        Ok(new_balance) => println!("UserAccount balance increased to {}", new_balance),
        Err(error_message) => println!("{}", error_message),
    }

    // use an if-let-else statement print UserAccount age if it is an Option::Some
    if let Some(age) = user_account.age {
        println!("User age is {}", age);
    } else {
        println!("User age is not set");
    }
}

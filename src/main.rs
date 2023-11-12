// Define a struct named UserAccount with fields name (String) and age (Option<u32>)
struct UserAccount {
    name: String,
    age: Option<u32>,
}

// Define a trait named Balance with a function get_balance returning an integer value of 10
trait Balance {
    fn get_balance(&self) -> u32 {
        10
    }
}

// Implement the Balance trait for the UserAccount struct
impl Balance for UserAccount {}

// Define a function increase_balance which takes a type implementing the Balance trait and a u32 amount
// This function returns a Result<u32, String>
fn increase_balance<T: Balance>(account: &T, amount: u32) -> Result<u32, String> {
    if amount <= 10 {
        // If the increase amount is <= 10, return an Ok with the sum of get_balance and the amount
        Ok(account.get_balance() + amount)
    } else {
        // If the increase amount is > 10, return an Err with a specific error message
        Err("Increase must be less than 10!".to_string())
    }
}

fn main() {
    // Create a UserAccount instance with the age set to Option::None
    let user_account = UserAccount {
        name: "John Doe".to_string(),
        age: None,
    };

    // Attempt to increase the user_account's balance by 11
    match increase_balance(&user_account, 11) {
        Ok(new_balance) => println!("UserAccount balance increased to {}", new_balance),
        Err(e) => println!("{}", e),
    }

    // Use an if...let...else statement to print the UserAccount age if it is Some
    if let Some(age) = user_account.age {
        println!("UserAccount age is {}", age);
    } else {
        println!("UserAccount age is not set");
    }
}

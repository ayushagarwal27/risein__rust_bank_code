fn main() {
   let mut john_bank_account = BankAccount{
       account_number:1120,
       holder_name:"John Amigo".to_string(),
       balance: 0.0
   };

    let mut alice_bank_account = BankAccount{
        account_number:2250,
        holder_name:"Alice Kentucky".to_string(),
        balance: 10.0
    };

    john_bank_account.depoist(500.5);
    alice_bank_account.withdraw(5.5);

    println!("John's balance: {}", john_bank_account.balance());
    println!("Alice's balance: {}", alice_bank_account.balance());
}

struct BankAccount{
    account_number: u32,
    holder_name:String,
    balance: f64
}

trait  Account  {
    fn depoist(& mut self, amount: f64);
    fn withdraw(& mut self, amount: f64);
    fn balance(&self) ->f64;
}

impl Account for BankAccount{
    fn depoist(& mut self, amount: f64) {
        self.balance += amount
    }

    fn withdraw(& mut self, amount: f64) {
        self.balance -= amount
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
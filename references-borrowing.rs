// Refrences and  Borrowing
// Safety and Performance

// References unable you to borrow values without ownership
// it can either mutable and immutable

fn main(){
    println!("=======================");
    let _x: i32 = 5;
    let _r: & i32 = &_x; 
    
    println!("The value of _x is {}",_x);
    println!("The value of _r is {} but _r is not the owner",_r);
    
    println!("=======================");
    
    let mut _a: i32 = 18;
    let _b: &mut i32 = &mut _a;
    
    *_b = *_b + 3;
    println!("The new value of _a is {} ",_a);
    
    println!("=======================");
    
    let mut account: BankAccount = BankAccount{
        owner: "Jemin".to_string(),
        balance: 1823.19,
    };
    
    // Immutable Borrow to check balance
    account.check_balance();
    
    // Mutabble borrow to withdraw money
    account.withdraw(50.72);
    
}

struct BankAccount{
    owner: String,
    balance: f64
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    
    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance)
    }
    
}


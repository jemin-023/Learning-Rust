// Ownership in Rust

// 1. Each value has a owner
// 2. There can  be only 1 owner at a time
// 3. When the owner goes out of scope the value is dropped

fn main(){
    let s1 = String::from("RUST"); //original Owner
    let len = calculate_lenght(&s1); // Borrowing
    println!("Lenth of  the String '{}' is {}",s1,len);
    
    let s2 = s1; //Transfered the ownership in s1 to a new variable called s2
    println!("{}",s2) // wont work with s1 now
}

//fn display(){
    //println!("s1 is {}",s1); Won't work!! Rule no. 3
//}

fn calculate_lenght(s: &String) -> usize{
    s.len()
}


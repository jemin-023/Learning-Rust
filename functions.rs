//Functions

//an functtion should be written in snake case: hello_world

fn main() {
    let u = 32;
    hello_world();
    tell_height(u);
    human("jemin",18,180.0);
    
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    
    println!("Result is {}", x);
    
    let k: i32 = add(3,5);
    
    println!("Sum = {}",k);
    
}

fn hello_world(){
    println!("Hello World!")
}

fn tell_height(height: i32){
    println!("Height is {}",height)
}

fn human(name: &str, age: u32, height: f32){
    println!("I'm {}, I am {} years old and im {} cm", name,age,height)
}

fn add(a:i32 , b:i32) -> i32{
    a+b
}

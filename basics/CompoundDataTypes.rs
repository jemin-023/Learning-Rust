// Compound data types
fn main() {
    //Array
    let number: [i32;5] =  [1,2,3,4,5];
    println!("Number  Array: {:?}", number);
    
    let language: [&str;3] = ["python","rust","cpp"];
    println!("First Language is {}", language[0]);
    
    //Tuples
    let human = ("Jemin",18,false);
    println!("Human Tupel: {:?}",human);
    
    //Slices: [1,2,3,4,5] Contigious
    let slice:&[i32] = &[1,2,3,4,5];
    println!("Slice: {:?}", slice);
    
    let animal :&[&str] = &["cat","dog"];
    println!("Animal: {:?}", animal);
    
    let name:&[&String] = &[&"jemin".to_string(),&"random".to_string()];
    println!("Name: {:?}",name);
    
    //String V/S String Slices (&str)
    let mut stone_cold: String = String::from("Hell, "); 
    println!("Stone Cold says: {}",stone_cold);
    //stone cold is in heap memeory
    
    stone_cold.push_str("Yeah");
    println!("Stone Cold says: {}",stone_cold);
    
    //&str (String Slice)  good for memory efficiency
    let string: String = String::from("Hellow, World");
    let string_slice: &str = &string[0..5];
    println!("Slice Value: {}", string_slice);    
 
    
}
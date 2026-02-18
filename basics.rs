//Primitive Data types
// int, float, char,  bool

fn main(){

    // let x: i32 = 18; //cant use a variable that is not initlized unlike c/c++ which give garbage
    // in rust a variable by nature is immutable unless specifed otherwise
    // x = 14; ERROR!!

  /*let mut y: i32 = 12;
    println!("y = {}\n",y);
    y = 23;
    println!("y = {}\n",y); */

    // unlike c++ this is not possible
    /* let s1 = String::from("hello");
    let s2 = s1;
    println!("{}",s1);*/


    //OWNERSHIP!!!!!!!!!!!!!!//
    // Every object has only one owner and when it goes out of scope the memory is freed

    /*let mut a =  10;
    let _b = 15;
    a = 12;
    println!("a = {}",a); */

    //=== 2 SUM from LeetCode(2 array variation) ===\\
    let arr1 = [1,2,3,4,5];
    let arr2 = [9,8,7,6,5];

    let target = 12;

    for i in 0..5 {
        for j in 0..5 {
            if arr1[i]+arr2[j] == target {
                println!("Target found! arr1[{}] + arr2[{}]",i,j);
                break;
            }
        }
    }

}
fn main() {
    // let x=5;
    let mut x=5;
    println!("The value of x is: {}",x);
    x=6; //shows error as x is immuatable
    println!("The value of x is: {}",x);


// //-----------------------------------------------

// //Constants: Always immutable

// const THREE_HOURS_IN_SECOND: u32=60*60*3;
// println!("The value of THREE_HOURS_IN_SECOND is: {} seconds",THREE_HOURS_IN_SECOND);

// //Shadowing

// fn main()
// {
//     let x=3;
//     let x=x+1;
//     //innerscope
//     {
//         let x=x*2;
//         println!("The value of x is: {}",x);
//     } 
//     println!("The value of x is: {}",x);
// }


fn main()
{
    // let x=3;
    // x=x+1; //cannot assign twice to immutable variable

    // let mut x="3";
    // x=1; //expected `&str`, found integer, We cann't mutate different data types (string and integers)

    // let mut firstname="Rajeeb";
    // firstname=firstname.len();
    // //expected `&str`, found `usize`

    // let firstname="Rajeeb";
    // let firstname=firstname.len(); //shadowing variables firstname

    let _firstname="Rajeeb";
    let _firstname=10;



    println!("The value of x is: {}",_firstname);
}


// }
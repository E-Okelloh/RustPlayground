// THE THREE RULES OF OWNERSHIP (tattoo these on your brain):
//   1. Each value has exactly ONE owner
//   2. When the owner goes out of scope, the value is DROPPED
//   3. There can only be ONE owner at a time

fn main () {

    let x: i32 = 5;
    let y = x;      //x is copied - 
}
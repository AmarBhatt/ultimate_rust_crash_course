const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    //let mut missiles: i32 = STARTING_MISSILES;
    //let ready: i32 = READY_AMOUNT;

    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    // Unused variable (will cause warning)
    //let unused = 3;

    // Changing the value of a constant is prohibited
    //STARTING_MISSILES = 12;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    // If you use the below line, a warning occurs because missiles no longer needs to be mutable.
    //println!("{} missiles left", missiles - ready);
}

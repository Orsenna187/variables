const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// constants are declared using const
// type must be specified (here u32)
// constants may only be set to a constant expression,
// not a value computed at runtime

fn main() {
    let y = 5; // variables are immutable by default
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {x}");

    }
    // x = 6;
    println!("The value of x is: {x}");
}

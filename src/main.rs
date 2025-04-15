const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// constants are declared using const. UPPERCASE convention.
// type must be specified (here u32)
// constants can be global. Good for hardcoded values.
// constants may only be set to a constant expression, not a value that can only be computed at runtime
// looks like it can do some compile-time computation. At least basic arithmetic. 

fn main() {
    let x = 5; 
    // x = 6; // this will not compile. Variables are immutable by default
    let x  = x + 1; // but variables can be shadowed.
    // shadowing is better than using mut. Because it needs the let keyword when changing a var. 
    // forced "let" = less likely to accidentally mutate something.

    let mut y = 5; // use mut to declare mutable variable
    y = 6;
    println!("The value of y is: {y}");

    // y = "foo"; // can't mutate the type of a mut.
    let y = "foo"; // but you can shadow bc it effectively creates a new var, just with the same name.
    {
        let x = x * 2; // shadowing is scoped.
        println!("The value of x is: {x}");

    }
    // x = 6;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

// Learnings
// - variables are immutable by default
// - can create a mutable variable with the mut keyword.
// - you can't mutate the type of a mut
// - variables can be shadowed. Which replaces them in scope.
// - shadowing effectively creates a new variable, letting you keep the namespace clean
// - shadowing is preferable to using mut bc it requires the let keyword. Less likely to accidentally mutate something.
// - constants are immutable, require a type, and can be global. 
// - constants can only be set to a constant expression that can be calculated at compile-time. Reminds me of macros in C.
// - constants are UPPERCASE by convention. 
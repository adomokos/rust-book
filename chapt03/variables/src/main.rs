use std::io;

fn main() {
    constants();
    shadowing();
    shadowing_typeswitch();
    tuples();
    arrays();
    array_element_finder();
}

fn array_element_finder() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant is: {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn arrays() {
    let a: [i32; 5] = [1,2,3,4,5];
    let last_element = a[4];
    println!("The last element of the array is {last_element}");
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _, _) = tup;
    let second_value = tup.1;

    println!("Tuple's first value is {x}");
    println!("Tuple's second value is {second_value}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn shadowing_typeswitch() {
    // Shadowing allows us to switch types
    let x = "something";

    {
        let x = x.len();
        println!("The length of 'something' is: {x}");
    }

    println!("The value of x is: {x}");
}

#[cfg(test)]
mod tests;

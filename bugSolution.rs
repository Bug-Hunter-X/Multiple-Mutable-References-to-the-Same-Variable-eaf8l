fn main() {
    let mut x = 5;
    { // Create a new scope for one mutable reference
        let y = &mut x;
        *y = 10;
    }

    { // Create a new scope for another mutable reference
        let z = &mut x;
        *z = 20; 
    }
    println!("x = {}", x);
} 
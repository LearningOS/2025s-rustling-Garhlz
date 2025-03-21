// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let price = 51;
    let ans = sale_price(price);
    println!("Your sale price is {}",ans);
}

fn sale_price(price : i32) -> i32 {
    if price & 1 == 1 {
        price - 3
    }
    else {
        price - 10
    }
}
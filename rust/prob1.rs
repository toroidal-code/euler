fn main() {
    let sum = range(0, 1000).filter( |x| { 
        x % 3 == 0 || x % 5 == 0
    }).fold( 0, |sum, tail| {
        sum + tail
    });
    
    println!("The sum of all numbers up to 1000 divisible by 3 or 5: {:i}", sum);
}
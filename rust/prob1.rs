fn main() {
	let sum = range(0, 1000).fold(0, |sum, x| { 
		if x % 3 == 0 || x % 5 == 0 {
			sum + x
		} else {
			sum
		}
	});
	println!("The sum of all numbers up to 1000 divisible by 3 or 5: {:i}", sum)
}
fn fib(n: int) -> int {
    fn _fib(n: int, a: int, b: int) -> int {
        match (n, a, b) {
            (0, _,  _) => a,
            _          => _fib(n - 1, a + b, a)
        }
    }

    _fib(n, 0, 1)
}

fn main() {
    let mut fibat = 1;
    let mut fibn = 0;
    let mut sum = 0;
    while fibn <= 4000000 {
        fibn = fib(fibat);
        fibat += 1;
        if fibn % 2 == 0 {
            sum += fibn
        };
    } 
    println!("{:i}", sum);
}





fn euler2(n: int, m: int, limit: int, total: int) -> int {
    match (n, m, limit, total) {
        (_, m, limit, total) if m > limit => total,
        (n, m, limit, total) => {
            match (n+m) % 2 {
                0 => euler2(m, n+m, limit, total+n+m),
                1 => euler2(m, n+m, limit, total),
                _ => { println!("Whoops"); 0 }
            }
        }
    }

}


fn main() {
    let sum = euler2(1, 2, 4000000, 2);
    println!("{:i}", sum);
}
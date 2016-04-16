pub struct Sieve;

impl Sieve {
    // I know I know, not a sieve... bit it is a oneliner!
    pub fn primes_up_to(n: u32) -> Vec<u32> {
        (2..).filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0)).take_while(|&i| i <= n).collect()
    }
}
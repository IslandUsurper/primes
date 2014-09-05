pub mod primes {
    use std::collections::TreeSet;
    use std::iter;

    pub fn is_prime(x: uint, primes: &TreeSet<uint>) -> bool {
        if x <= 3 {
            x > 1
        }
        else {
            if x % 2 == 0 || x % 3 == 0 {
                return false;
            }
            for i in primes.iter() {
                if x % *i == 0 {
                    return false;
                }
            }
            let last = match primes.rev_iter().next() {
                Some(x) => *x,
                    None    => 5u,
            };
            for i in iter::range_step(last, (x as f32).sqrt() as uint + 1, 6) {
                if x % i == 0 || x % (i + 2) == 0 {
                    return false;
                }
            }
            true
        }
    }

    pub fn get_primes(n: uint) -> TreeSet<uint> {
        let mut primes: TreeSet<uint> = [2u, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter().map(|&x| x).collect();
        let mut counter = iter::count(29u, 2);

        while primes.len() < n {
            match counter.next() {
                Some(x) => {
                    if is_prime(x, &primes) {
                        primes.insert(x);
                    }
                },
                    None    => fail!("Ran out of numbers!"),
            };
        }

        primes
    }

    #[test]
    fn zero_is_not_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(0u, &primes), false);
    }

    #[test]
    fn one_is_not_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(1u, &primes), false);
    }

    #[test]
    fn two_is_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(2u, &primes), true);
    }

    #[test]
    fn three_is_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(3u, &primes), true);
    }

    #[test]
    fn four_is_not_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(4u, &primes), false);
    }

    #[test]
    fn five_is_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(5u, &primes), true);
    }

    #[test]
    fn thirty_one_is_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(31u, &primes), true);
    }

    #[test]
    fn thirty_three_is_not_prime() {
        let primes: TreeSet<uint> = TreeSet::new();
        assert_eq!(is_prime(33u, &primes), false);
    }

    #[test]
    fn get_primes_gives_primes() {
        let primes = get_primes(20);
        let empty: TreeSet<uint> = TreeSet::new();
        for i in primes.iter() {
            assert_eq!(is_prime(*i, &empty), true);
        }
    }

}

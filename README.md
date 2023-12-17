# Sieve Of Atkin Algorithm in Rust

This is an implementation of the Sieve of Atkin, a modern algorithm to find all prime numbers up to a certain integer.

## Why?
I embarked on a journey to relearn math a few weeks ago, starting with 8th-grade math and recently advancing to pre-algebra, where I was introduced to prime numbers. I somewhat knew what prime numbers were (don't make fun of me), but I never paid much attention in math class, so I couldn't really tell you which numbers are prime.

Long story short, I found it tedious to try and find prime numbers by hand when doing practice problems. This led me to search for more efficient ways to find prime numbers, which then led me to discover the Sieve of Atkin Algorithm. From there, I decided to implement the algorithm in code, both to automate the process and to explore more with Rust.

It was a fun project. I used the explanation on [Wikipedia](https://en.wikipedia.org/wiki/Sieve_of_Atkin) for guidance. There is an example with pseudocode, but it wasn't very helpful.

## Explanation
The Sieve of Atkin is an optimized algorithm used to find all prime numbers up to a specified integer limit. It's considered more efficient than the classical Sieve of Eratosthenes, especially for large numbers. The algorithm works by initially assuming all numbers are non-prime and then systematically flips the status of multiple specific numbers based on a set of mathematical rules. Only numbers less than or equal to the square root of the specified limit are considered. The algorithm efficiently identifies primes by eliminating non-prime numbers based on quadratic equations.

## Usage
Simply clone this repo and execute `cargo run` to run the program. You'll see all the prime numbers in the given array (located in `main.rs`). The code is well documented for those who wish to understand what it does and how each part works.

## Contributing
If you find anything that I might have done incorrectly (which I shouldn't have), feel free to make a pull request with changes.

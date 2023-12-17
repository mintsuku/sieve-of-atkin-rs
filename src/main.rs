//program entry point
fn main() {
    process_sieve_list();
}


// Processes a list of numbers by applying sieve of atkins algorithm.
// It iterates through each number in the sieve_list and processes them.
fn process_sieve_list() {
    let sieve_list = vec![
        107, 279, 472, 989, 985, 94, 887, 173, 291, 726, 914, 519, 305, 439, 115, 560, 410, 440,
        380, 103, 222, 603, 692, 875, 263, 834, 864, 31, 901, 386, 959, 414, 631, 931, 374, 304,
        1000, 520, 576, 671, 514, 53, 437, 692, 461, 343, 929, 805, 22, 572, 326, 464, 561, 384,
        285, 692, 621, 345, 575, 400, 115, 758, 210, 692, 962, 656, 626, 560, 751, 881, 853, 63,
        632, 69, 980, 655, 984, 768, 694, 571, 510, 723, 998, 398, 546, 863, 963, 219, 331, 656,
        416, 289, 479, 119, 761, 262, 972, 766, 106, 696,
    ];

    // Iterating through each number in the sieve_list
    for &number in &sieve_list {
        process_number(number);
    }
}

// Processes an individual number to determine if it's prime.
// This function finds the remainder of the number modulo 60, then
// applies different solution sets based on the remainder. 
fn process_number(number: i32) {
    let remainder = number % 60;
    let odd_solution_sets = vec![
        (vec![1, 13, 17, 29, 37, 41, 49, 53], odd_solutions_quadratic as fn(i32) -> i32),
        (vec![7, 19, 31, 43], odd_solutions_three as fn(i32) -> i32),
        (vec![7, 19, 31, 43], odd_solutions_minus as fn(i32) -> i32),
    ];

    for (set, solver_function) in odd_solution_sets {
        if set.contains(&remainder) {
            let solutions = solver_function(number);
            if is_square_free(number) && solutions % 2 == 1 {
                println!("{} is a prime", number);
            }
            break;
        }
    }
}


// Calculates the number of odd solutions for a given number using the quadratic equation.
fn odd_solutions_quadratic(n: i32) -> i32 {
    count_odd_solutions(n, |x, y| 4 * x * x + y * y)
}

// Calculates the number of odd solutions for a given number using a specific equation.
fn odd_solutions_three(n: i32) -> i32 {
    count_odd_solutions(n, |x, y| 3 * x * x + y * y)
}

// Calculates the number of odd solutions for a given number using a modified equation.
fn odd_solutions_minus(n: i32) -> i32 {
    count_odd_solutions(n, |x, y| 3 * x * x - y * y)
}

// Counts the number of odd solutions for a given number using a provided equation.
// It iterates through all possible values of x and y to find solutions to the equation.
fn count_odd_solutions<F>(n: i32, equation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let mut count = 0;
    // Iterating through potential solutions
    for x in 0..n {
        for y in 0..n {
            // Counting odd solutions
            if equation(x, y) == n && (x + y) % 2 == 1 {
                count += 1;
            }
        }
    }
    count
}

// Checks if a number is square-free (not divisible by any square number other than 1).
// This is used to aid in determining if a number is prime.
fn is_square_free(n: i32) -> bool {
    let sqrt_n = (n as f64).sqrt() as i32;
    // Checking divisibility by squares of numbers up to its square root
    for i in 2..=sqrt_n {
        if n % (i * i) == 0 {
            return false;
        }
    }
    true
}
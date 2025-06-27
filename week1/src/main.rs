fn main() {
    let array_size = 10;
    let fibonacci_array = generate_fibonacci_array(array_size);

    for value in fibonacci_array {
        println!("{}", value);
    }
}

fn generate_fibonacci_array(array_size: usize) -> Vec<u64> {
    let mut fib_array: Vec<u64> = vec![0; array_size];

    if array_size > 0 {
        fib_array[0] = 0;
    }
    if array_size > 1 {
        fib_array[1] = 1;
    }

    for i in 2..array_size {
        fib_array[i] = fib_array[i - 1] + fib_array[i - 2];
    }

    fib_array
}

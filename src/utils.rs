/// The mod operator (works correctly with negatives)
pub fn modulo(input: i32, divisor: i32) -> i32 {
    ((input % divisor) + divisor) % divisor
}

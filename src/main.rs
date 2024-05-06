fn main() {
    // This is a comment
    #[cfg(feature = "llvm-18")]
    println!("Hello, world!");
    #[cfg(feature = "llvm-17")]
    println!("Hello, world!");
}


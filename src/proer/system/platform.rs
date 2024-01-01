pub trait Platform {
    fn new() -> Self;
    fn deinit(self: &mut Self) {}
}
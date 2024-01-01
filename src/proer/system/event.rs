#[derive(Debug)]
pub enum Event {
    Close,
    Resize ((u32, u32)),
}
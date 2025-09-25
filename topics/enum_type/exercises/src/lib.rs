// This macro is needed to be able to print the enum, and to be able to compare enum values, respectively:
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32)
}

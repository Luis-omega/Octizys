pub mod error;
pub mod identifier;
pub mod module_logic_path;
pub mod newtype;
pub mod span;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

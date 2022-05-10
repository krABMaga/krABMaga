/// Enum to select how to apply closure to grid
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum GridOption {
    /// Apply closure from Read state
    READ,
    /// Apply closure from Write state
    WRITE,
    /// Apply closure reading from Read and writing into Write state
    READWRITE,
}

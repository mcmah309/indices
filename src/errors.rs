/// The error type returned from the `try_indices!` macro.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TryIndicesError {
    /// The requested index is larger than the length of the input slice.
    IndexOutOfBounds,
    /// The index has been requested twice.
    DuplicateIndex,
}

impl std::error::Error for TryIndicesError {}

impl core::fmt::Display for TryIndicesError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            TryIndicesError::IndexOutOfBounds => "TryIndicesError::IndexOutOfBounds",
            TryIndicesError::DuplicateIndex => "TryIndicesError::DuplicateIndex",
        };
        write!(f, "{}", variant_name)
    }
}

/// The error type returned from the `try_indices_ordered!` macro.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TryIndicesOrderedError {
    /// The requested index is larger than the length of the input slice.
    IndexOutOfBounds,
    /// The input indices are either not sorted or out of bounds.
    InvalidIndex,
}

impl std::error::Error for TryIndicesOrderedError {}

impl core::fmt::Display for TryIndicesOrderedError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            TryIndicesOrderedError::IndexOutOfBounds => "TryIndicesOrderedError::IndexOutOfBounds",
            TryIndicesOrderedError::InvalidIndex => "TryIndicesOrderedError::InvalidIndex",
        };
        write!(f, "{}", variant_name)
    }
}

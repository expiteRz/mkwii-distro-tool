pub trait ExistSubstringExt {
    /// Check if substring exists in vector of u16 (UTF-16 texts)
    fn check_substring_exist(&self, sub: &[u16]) -> Option<usize>;
}

pub trait AdjustLengthExt {
    fn adjust_length(&self) -> usize;
}
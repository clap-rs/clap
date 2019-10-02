pub trait _StrExt {
    fn _is_char_boundary(&self, index: usize) -> bool;
}

impl _StrExt for str {
    #[inline]
    fn _is_char_boundary(&self, index: usize) -> bool {
        if index == self.len() {
            return true;
        }

        self.as_bytes()
            .get(index)
            .map_or(false, |&b| b < 128 || b >= 192)
    }
}

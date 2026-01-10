use std::ops::Range;

pub(crate) struct Cursor<'a> {
    src: &'a str,
    idx: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) const fn new(src: &'a str) -> Self {
        Self { src, idx: 0 }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = (Range<usize>, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.src.len() {
            return None;
        }

        let substr = unsafe { self.src.get_unchecked(self.idx..) };

        let new_line = match substr.find('\n') {
            Some(some) => some,
            None => {
                let range = self.idx..self.src.len();
                let res = Some((range, substr));
                self.idx = self.src.len();
                return res;
            }
        };

        let end = self.idx + new_line;
        let range = self.idx..end;
        self.idx = end + 1;

        let line = unsafe { substr.get_unchecked(..new_line) };
        let line = line.strip_suffix('\r').unwrap_or(line);

        Some((range, line))
    }
}

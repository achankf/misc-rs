use super::Buf2;
use std::string::ToString;

impl Buf2 {
    /// Create a new instance of Buf2.
    pub fn new(row: usize, col: usize) -> Self {
        Buf2 {
            row: row,
            col: col,
            crow: 0,
            ccol: 0,
            buf: vec![' '; row * col],
        }
    }

    /// Return the dimension of the buffer.
    pub fn dim(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    /// Return the location of the cursor.
    pub fn cloc(&self) -> (usize, usize) {
        (self.crow, self.ccol)
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.col + col
    }

    /// Put a character, relative to the cursor.
    pub fn put(&mut self, c: char) {
        let (crow, ccol) = self.cloc();
        self.put_at(crow, ccol, c);
    }

    /// Put a character at the specified location.
    pub fn put_at(&mut self, row: usize, col: usize, c: char) {
        let idx = self.idx(row, col);
        self.buf[idx] = c; // let bad indices cause overflow and panic
        self.mv_next();
    }

    /// Move the cursor to the specified location.
    pub fn mv(&mut self, crow: usize, ccol: usize) {
        self.crow = crow;
        self.ccol = ccol;
    }

    /// Move the cursor to the next position (as in how you would read from left to right, top to bottom).
    pub fn mv_next(&mut self) {
        let ccol = (self.ccol + 1) % self.col;
        let crow = if ccol == 0 {
            self.crow + 1
        } else {
            self.crow
        };
        self.mv(crow, ccol);
    }

    /// Write the object as a string, relative to the cursor.
    pub fn write<T>(&mut self, o: &T)
        where T: ToString
    {
        let (crow, ccol) = self.cloc();
        self.write_at(o, crow, ccol);
    }

    /// Write the object as a string, to the next position.
    pub fn write_at<T>(&mut self, o: &T, crow: usize, ccol: usize)
        where T: ToString
    {
        self.mv(crow, ccol);
        self.write_str_at(&o.to_string(), crow, ccol);
    }

    /// Write the buffer at a specific location.
    pub fn write_buf2_at(&mut self, buf: &Buf2, crow: usize, ccol: usize) {
        let (parent_row, parent_col) = self.dim();
        let (_, child_col) = buf.dim();

        let mut iter = buf.buf.iter().peekable();
        let mut acrow = crow;

        loop {

            if acrow >= parent_row {
                break;
            }

            let mut accol = ccol;
            for c in iter.by_ref().take(child_col) {
                if accol >= parent_col {
                    // consume the iterator
                    continue;
                }
                let idx = self.idx(acrow, accol);
                self.buf[idx] = *c;
                accol += 1;
            }
            acrow += 1;

            // if iter.by_ref().is_empty() { // not stable right now
            if iter.by_ref().peek() == None {
                break;
            }
        }
    }

    /// Write a string, relative to the cursor.
    pub fn write_str(&mut self, s: &str) {
        let (crow, ccol) = self.cloc();
        self.write_str_at(s, crow, ccol);
    }

    /// Write a string at the specified location.
    pub fn write_str_at(&mut self, s: &str, crow: usize, ccol: usize) {
        self.mv(crow, ccol);
        for c in s.chars() {
            self.put(c);
        }
    }

    /// Clear the buffer with empty spaces.
    pub fn clear(&mut self) {
        self.buf.clear();
        for _ in 0..self.row * self.col {
            self.buf.push(' ');
        }
    }
}

impl ToString for Buf2 {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        let mut ccol = 0;

        for c in self.buf.iter() {
            ret.push(*c);
            ccol += 1;
            if ccol == self.col {
                ret.push('\n');
                ccol = 0;
            }
        }
        ret
    }
}

impl Default for Buf2 {
    fn default() -> Self {
        // roughly the size of a terminal screen
        Buf2::new(20, 80)
    }
}

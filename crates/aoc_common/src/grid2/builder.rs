use crate::{Grid2, file_lines};

pub struct Grid2Builder<T> {
    width: Option<usize>,
    height: Option<usize>,
    buffer: Vec<Vec<T>>,
}

impl Grid2Builder<u8> {
    pub fn from_file_as_bytes(path: &str) -> Grid2<u8> {
        let mut lines = file_lines(path).peekable();

        let first = lines.peek();
        if first.is_none() {
            panic!("empty file");
        }
        let first = first.unwrap();
        let width = first.len();
        if width == 0 {
            panic!("empty file")
        }

        // this might blow up, but assume a square-ish input
        let mut buffer: Vec<u8> = Vec::with_capacity(width * width);
        let mut height = 0;

        for line in lines {
            if line.len() != width {
                panic!("jagged lines");
            }

            height += 1;
            buffer.extend(line.into_bytes().into_iter());
        }

        buffer.shrink_to_fit();

        Grid2 {
            width,
            height,
            buffer
        }
    }
}
pub struct Matrix<T> {
    buff: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        if rows == 0 { panic!(); }
        for row in value.iter() {
            if row.len() != rows {
                panic!();
            }
        }

        Self {
            buff: value,
        }
    }
}
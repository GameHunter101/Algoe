use num_traits::Float;

#[derive(Debug)]
struct Vector<T, const DIM: usize> {
    data: [T;DIM]
}

/* impl<T: Float, const DIM: usize> Vector<T, DIM> {
    pub fn wedge(&self, rhs: Self) -> () {

    }
} */

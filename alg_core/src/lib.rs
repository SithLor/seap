use num_traits::Zero;

fn make_graph<T: Zero + Clone>(rows: usize, cols: usize) -> Vec<Vec<T>> {
    vec![vec![T::zero(); cols]; rows]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

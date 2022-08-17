mod shadowing;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::shadowing::{shadowing_foo, shadowing_foo1};
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_shadowing() {
        shadowing_foo()
    }

    #[test]
    fn test_shadowing1() {
        shadowing_foo1()
    }
}

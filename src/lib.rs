pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::cmp::PartialOrd> Rect<T> {
    fn can_hold(&self, other: &Rect<T>) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explore() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn small_hold_big_rect(){
        let larger = Rect {
            width: 5,
            height: 6,
        };
        let smaller = Rect {
            width: 1,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }
}

struct Rectangle {
    width: u32,
    length: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, smaller: &Rectangle) -> bool {
        let volume_large = self.width * self.height * self.length;
        let volume_small = smaller.width * smaller.height * smaller.length;

        if volume_large > volume_small {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_rect() {
        let smaller = Rectangle {
            width: 5,
            length: 9,
            height: 4,
        };

        let larger = Rectangle {
            width: 9,
            length: 6,
            height: 4,
        };

        assert!(larger.can_hold(&smaller),"Rectangle cannot hold smaller");
    }
}

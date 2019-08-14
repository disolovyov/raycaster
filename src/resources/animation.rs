#[derive(Default)]
pub struct Animation {
    tick: u64,
}

impl Animation {
    pub fn advance(&mut self) {
        self.tick += 1;
    }

    pub fn frame(&self, ticks_per_frame: u8, total_frames: u8) -> u8 {
        (self.tick / u64::from(ticks_per_frame) % u64::from(total_frames)) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn frame() {
        let mut animation = Animation::default();

        assert_eq!(animation.frame(3, 3), 0);

        animation.advance();
        animation.advance();

        assert_eq!(animation.frame(3, 3), 0);

        animation.advance();
        animation.advance();

        assert_eq!(animation.frame(3, 3), 1);

        for _ in 0..12 {
            animation.advance();
        }

        assert_eq!(animation.frame(3, 3), 2);
    }
}

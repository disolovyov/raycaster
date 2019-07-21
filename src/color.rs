pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    rgba(r, g, b, 255)
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let ac = (a as u32) << 24;
    let bc = (b as u32) << 16;
    let gc = (g as u32) << 8;
    let rc = r as u32;
    ac + bc + gc + rc
}

pub fn unpack_rgba(rgba: u32) -> (u8, u8, u8, u8) {
    let r = (rgba >> 0) as u8;
    let g = (rgba >> 8) as u8;
    let b = (rgba >> 16) as u8;
    let a = (rgba >> 24) as u8;
    (r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::rgb;
    use super::rgba;
    use super::unpack_rgba;

    #[test]
    fn test_rgb() {
        let c = rgb(108, 113, 196);
        let uc = unpack_rgba(c);

        assert_eq!(uc, (108, 113, 196, 255));
    }

    #[test]
    fn test_rgba() {
        let c = rgba(108, 113, 196, 128);
        let uc = unpack_rgba(c);

        assert_eq!(uc, (108, 113, 196, 128));
    }
}

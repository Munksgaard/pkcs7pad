pub fn pad(input: &[u8], bsize: u8) -> Vec<u8> {
    let mut input = input.to_vec();
    let padding = if input.len() % (bsize as usize) == 0 {
        bsize
    } else {
        bsize - ((input.len() % (bsize as usize)) as u8)
    };
    let len = input.len();

    input.resize(len + padding as usize, padding);
    input
}

pub fn validate_padding(input: &[u8]) -> bool {
    let last = match input.last() {
        Some(x) => *x,
        None => return false,
    };

    if last > input.len() as u8 || last == 0 {
        return false;
    }

    input
        .iter()
        .skip(input.len() - (last as usize))
        .all(|x: &u8| *x == last)
}

pub fn unpad(input: &[u8]) -> Option<Vec<u8>> {
    if !validate_padding(input) {
        return None;
    }

    let last = *input.last().unwrap();

    let mut result = input.to_vec();

    result.truncate(input.len() - last as usize);
    Some(result)
}

#[cfg(test)]
mod test {
    use super::pad;
    use super::validate_padding;
    use super::unpad;

    #[test]
    fn pad_test_1() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E];
        let bsize = 16;
        let expected = vec![0x59, 0x45, 0x4C, 0x4C,
                            0x4F, 0x57, 0x20, 0x53,
                            0x42, 0x55, 0x4D, 0x41,
                            0x49, 0x52, 0x4E, 0x01];
        assert_eq!(pad(input, bsize), expected);
    }

    #[test]
    fn pad_test_2() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x45];
        let bsize = 16;
        let expected = vec![0x59, 0x45, 0x4C, 0x4C,
                            0x4F, 0x57, 0x20, 0x53,
                            0x42, 0x55, 0x4D, 0x41,
                            0x49, 0x52, 0x4E, 0x45,
                            0x10, 0x10, 0x10, 0x10,
                            0x10, 0x10, 0x10, 0x10,
                            0x10, 0x10, 0x10, 0x10,
                            0x10, 0x10, 0x10, 0x10];
        assert_eq!(pad(input, bsize), expected);
    }

    #[test]
    fn validate_padding_test_1() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x45,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10];
        assert!(validate_padding(input));
    }

    #[test]
    fn validate_padding_test_2() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x45,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10];
        assert!(!validate_padding(input));
    }

    #[test]
    fn validate_padding_test_3() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x45,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x0A,
                      0x10, 0x10, 0x10, 0x10];
        assert!(!validate_padding(input));
    }

    #[test]
    fn validate_padding_test_4() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x45,
                      0xFF, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10];
        assert!(!validate_padding(input));
    }

    #[test]
    fn validate_padding_test_5() {
        let input = &[0xFF];
        assert!(!validate_padding(input));
    }

    #[test]
    fn validate_padding_test_6() {
        let input = &[0x00, 0x00, 0x00, 0x00,
                      0x00, 0x00, 0x00, 0x00,
                      0x00, 0x00, 0x00, 0x00,
                      0x00, 0x00, 0x00, 0x00];
        assert!(!validate_padding(input));
    }

    #[test]
    fn unpad_test_1() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x01];
        let expected = vec![0x59, 0x45, 0x4C, 0x4C,
                            0x4F, 0x57, 0x20, 0x53,
                            0x42, 0x55, 0x4D, 0x41,
                            0x49, 0x52, 0x4E];
        assert_eq!(unpad(input), Some(expected));
    }

    #[test]
    fn unpad_test_2() {
        let input = &[0x59, 0x45, 0x4C, 0x4C,
                      0x4F, 0x57, 0x20, 0x53,
                      0x42, 0x55, 0x4D, 0x41,
                      0x49, 0x52, 0x4E, 0x45,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10,
                      0x10, 0x10, 0x10, 0x10];
        let expected = vec![0x59, 0x45, 0x4C, 0x4C,
                            0x4F, 0x57, 0x20, 0x53,
                            0x42, 0x55, 0x4D, 0x41,
                            0x49, 0x52, 0x4E, 0x45];
        assert_eq!(unpad(input), Some(expected));
    }

    #[test]
    fn chal15_validate_1() {
        let input = b"ICE ICE BABY\x04\x04\x04\x04";

        assert!(validate_padding(input));
        assert_eq!(unpad(input).unwrap(), b"ICE ICE BABY");
    }

    #[test]
    fn chal15_validate_2() {
        let input = b"ICE ICE BABY\x05\x05\x05\x05";
        assert!(!validate_padding(input));
        assert_eq!(None, unpad(input));
    }

    #[test]
    fn chal15_validate_3() {
        let input = b"ICE ICE BABY\x01\x02\x03\x04";
        assert!(!validate_padding(input));
        assert_eq!(None, unpad(input));
    }
}

pub fn pad(input: &[u8], bsize: u8) -> Vec<u8> {
    let mut input = input.to_vec();
    let padding = if input.len() % (bsize as uint) == 0 {
        bsize
    } else {
        bsize - ((input.len() % (bsize as uint)) as u8)
    };

    input.grow(padding as uint, padding);
    input
}

#[cfg(test)]
mod test {
    use super::pad;

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
}

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

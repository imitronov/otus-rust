struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

impl RingBuffer {
    fn create(size: usize) -> RingBuffer {
        RingBuffer {
            read_idx: 0,
            write_idx: 0,
            data: vec![0; size],
        }
    }

    fn write(rb: &mut RingBuffer, data: &str) -> Result<usize, String> {
        let bytes = data.as_bytes();
        let mut written = 0;

        for &byte in bytes {
            if rb.data[rb.write_idx] != 0 {
                if written == 0 {
                    return Err("NoSpaceLeft".to_owned());
                } else {
                    break;
                }
            }

            rb.data[rb.write_idx] = byte;
            rb.write_idx += 1;

            if rb.write_idx >= rb.data.len() {
                rb.write_idx = 0;
            }

            written += 1;
        }

        Ok(written)
    }

    fn read(rb: &mut RingBuffer, count: usize) -> Option<String> {
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            if rb.data[rb.read_idx] == 0 {
                if result.is_empty() {
                    return None;
                } else {
                    break;
                }
            }

            result.push(rb.data[rb.read_idx]);
            rb.data[rb.read_idx] = 0;
            rb.read_idx += 1;

            if rb.read_idx >= rb.data.len() {
                rb.read_idx = 0;
            }
        }

        String::from_utf8(result).ok()
    }
}

fn main() {
    let mut rb = RingBuffer::create(4);

    let written = RingBuffer::write(&mut rb, "abcd");
    let result = RingBuffer::read(&mut rb, 4);

    println!("{:?}", written);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut rb = RingBuffer::create(3);
        assert_eq!(RingBuffer::write(&mut rb, "ab"), Ok(2));
        assert_eq!(RingBuffer::write(&mut rb, "cd"), Ok(1));
        assert_eq!(RingBuffer::read(&mut rb, 1), Some("a".to_string()));
        assert_eq!(RingBuffer::write(&mut rb, "e"), Ok(1));
        assert_eq!(RingBuffer::read(&mut rb, 2), Some("bc".to_string()));
    }
}

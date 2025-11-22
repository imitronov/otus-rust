use std::sync::{Arc, Mutex};

struct RingBufferData {
    read_idx: usize,
    write_idx: usize,
    buffer: Vec<u8>,
}

struct RingBuffer {
    data: Arc<Mutex<RingBufferData>>,
}

impl RingBuffer {
    fn create(size: usize) -> RingBuffer {
        RingBuffer {
            data: Arc::new(Mutex::new(RingBufferData {
                read_idx: 0,
                write_idx: 0,
                buffer: vec![0; size],
            })),
        }
    }

    fn write(&self, data: &str) -> Result<usize, String> {
        let bytes = data.as_bytes();
        let mut written = 0;

        let rb = Arc::clone(&self.data);
        let mut rb = rb.lock().unwrap();

        for &byte in bytes {
            let write_id = rb.write_idx;

            if rb.buffer[rb.write_idx] != 0 {
                if written == 0 {
                    return Err("NoSpaceLeft".to_owned());
                } else {
                    break;
                }
            }

            rb.buffer[write_id] = byte;
            rb.write_idx += 1;

            if rb.write_idx >= rb.buffer.len() {
                rb.write_idx = 0;
            }

            written += 1;
        }

        Ok(written)
    }

    fn read(&self, count: usize) -> Option<String> {
        let mut result = Vec::with_capacity(count);

        let rb = Arc::clone(&self.data);
        let mut rb = rb.lock().unwrap();

        for _ in 0..count {
            let read_idx = rb.read_idx;

            if rb.buffer[rb.read_idx] == 0 {
                if result.is_empty() {
                    return None;
                } else {
                    break;
                }
            }

            result.push(rb.buffer[rb.read_idx]);
            rb.buffer[read_idx] = 0;
            rb.read_idx += 1;

            if rb.read_idx >= rb.buffer.len() {
                rb.read_idx = 0;
            }
        }

        String::from_utf8(result).ok()
    }
}

fn main() {
    let rb = RingBuffer::create(4);

    let written = rb.write("abcd");
    let result = rb.read(4);

    println!("{:?}", written);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let rb = RingBuffer::create(3);
        assert_eq!(rb.write("ab"), Ok(2));
        assert_eq!(rb.write("cd"), Ok(1));
        assert_eq!(rb.read(1), Some("a".to_string()));
        assert_eq!(rb.write("e"), Ok(1));
        assert_eq!(rb.read(2), Some("bc".to_string()));
    }

    #[test]
    fn multithread_stress_test() {
        use std::sync::Arc;
        use std::thread;

        let word = "foo";
        let word_len = word.len();
        let clones = 10;
        let cycles = 10_000;
        let rb: Arc<RingBuffer> = Arc::new(RingBuffer::create(word_len * clones * cycles));

        let mut handles = vec![];

        for _ in 0..clones {
            let rb = rb.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..cycles {
                    let _ = rb.write(word);
                }
            }));
        }

        for _ in 0..clones {
            let rb = rb.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..cycles {
                    let out = rb.read(3);

                    if let Some(s) = out {
                        assert_eq!(s, word);
                    }
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let out = rb.read(word_len);

        assert_eq!(out, None);
    }
}

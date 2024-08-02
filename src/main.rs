use std::io::{self, Read, Write, BufReader, Cursor};
const MIN_CHUNK_SIZE: usize = 1;
const MAX_CHUNK_SIZE: usize = 90;
const MASK: u32 = 0x0FF;

fn rolling_hash(data: &[u8]) -> u32 {
    let mut hash = 0u32;
    for &byte in data {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
    }
    hash
}

// QuickCDC algorithm implementation
fn quickcdc<R: Read>(input: R, output: &mut dyn Write) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut start = 0;
    while start < buffer.len() {
        let end = (start + MAX_CHUNK_SIZE).min(buffer.len());
        let mut cut_point = end;

        // Search for a cut point within the buffer
        for i in (start + MIN_CHUNK_SIZE)..end {
            let window_end = i + 32; // Window size for hash calculation
            if window_end <= buffer.len() {
                let hash = rolling_hash(&buffer[i..window_end]);
                if hash & MASK == 0 {
                    cut_point = i;
                    break;
                }
            }
        }

        // Write the chunk to the output
        output.write_all(&buffer[start..cut_point])?;
        output.write_all(b"\n---\n")?; // Separator between chunks for clarity
        start = cut_point;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let input_data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque vehicula. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque vehicula. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque vehicula. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque vehicula.";

    let input = Cursor::new(input_data);
    let mut output = Vec::new();

    quickcdc(input, &mut output)?;

    println!("{}", String::from_utf8_lossy(&output));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash() {
        let data = b"Hello, world!";
        let hash = rolling_hash(data);
        assert_eq!(hash, 2414922741);
    }
    #[test]
    fn test_quickcdc() {
        let input_data = b"ishd dshvl ";
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        quickcdc(input, &mut output).unwrap();

        let output_str = String::from_utf8_lossy(&output);
        let chunks: Vec<&str> = output_str.split("\n---\n").collect();

        // Проверка, что результат делится на несколько частей
        assert!(chunks.len() > 1);

        // Проверка, что все куски вместе дают исходные данные
        let mut reconstructed = String::new();
        for chunk in &chunks {
            reconstructed.push_str(chunk);
        }
        reconstructed.truncate(reconstructed.len() - 1); // удалить последний '\n'
        assert_eq!(reconstructed.as_bytes(), input_data);
    }
}
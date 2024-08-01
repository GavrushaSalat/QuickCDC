use std::io::{self, Read, Write, BufReader};

// Parameters for the QuickCDC algorithm
const MIN_CHUNK_SIZE: usize = 2048;
const MAX_CHUNK_SIZE: usize = 8192;
const MASK: u32 = 0x1FFF;

fn rolling_hash(data: &[u8]) -> u32 //Rabin-Karp algorithm
{
    let mut hash = 0u32;
    for &byte in data {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
    }
    hash
}

// QuickCDC algorithm implementation
fn quickcdc <R: Read>(input: R, output: &mut dyn Write) -> io::Result<()>
{
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

use std::io::{stdout, BufWriter, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Maximum efficiency for outputting the answer
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);
    writer.write_all(joe::who_is_joe().as_bytes())?;
    writer.write_all(&[b'\n'])?;
    writer.flush()?;
    Ok(())
}

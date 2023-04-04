use std::io::{stdout, BufWriter, Write};
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    
    if let Some(arg) = args.next() {
        if arg == "--version" {
            println!("joe --version? more like joe mama!");
            return Ok(());
        }
    }

    // Maximum efficiency for outputting the answer
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);
    writer.write_all(joe::who_is_joe().as_bytes())?;
    writer.write_all(&[b'\n'])?;
    writer.flush()?;
    Ok(())
}

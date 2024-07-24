use color_eyre::eyre::Result;
use memmap2::Mmap;

fn load_file(filename: &str) -> Result<Mmap> {
    Ok(unsafe { Mmap::map(&std::fs::File::open(filename)?)? })
}

fn read(file: Mmap) -> Result<()> {
    println!("READ {:?}", &file[..]);
    Ok(())
}

fn parse_avx(data: &[u8], offset: usize) {}

fn main() -> Result<()> {
    color_eyre::install()?;

    let file = std::fs::File::open("src/main.rs")?;

    let mmap = unsafe { Mmap::map(&file)? };

    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use testresult::TestResult;

    use super::*;

    #[test]
    fn test_1() -> TestResult {
        let file = load_file("samples/measurements-1.txt")?;
        read(file)?;
        Ok(())
    }
}

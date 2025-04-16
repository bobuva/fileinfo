use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
pub enum ExecutableType {
    ELF,
    PE,
    Unknown,
}

pub fn determine_executable_type(file_name: &str) -> io::Result<ExecutableType> {
    let mut file = File::open(file_name)?;
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;

    // Check for ELF magic number (0x7F followed by "ELF")
    if buffer == [0x7F, b'E', b'L', b'F'] {
        println!("ELF magic number found: {:?}", buffer);
        return Ok(ExecutableType::ELF);
    }

    // Check for PE magic number (MZ header)
    if buffer[0] == b'M' && buffer[1] == b'Z' {
        println!("PE magic number found: {:?}", buffer);
        return Ok(ExecutableType::PE);
    }

    Ok(ExecutableType::Unknown)
}
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum ExecutableType {
    ELF,
    PE,
    Unknown,
}

#[derive(Debug)]
pub struct PEHeaders {
    pub dos_header: [u8; 64], // DOS header (first 64 bytes)
    pub dos_stub: Vec<u8>,    // DOS stub (variable size)
    pub nt_headers: Vec<u8>,  // NT headers (variable size)
    pub section_headers: Vec<u8>, // Section Table headers (variable size)
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

pub fn parse_pe_headers(file_name: &str) -> io::Result<PEHeaders> {
    let mut file = File::open(file_name)?;

    // Read DOS header (first 64 bytes)
    let mut dos_header = [0u8; 64];
    file.read_exact(&mut dos_header)?;

    // Read the e_lfanew field (offset to NT headers)
    let e_lfanew = u32::from_le_bytes([dos_header[60], dos_header[61], dos_header[62], dos_header[63]]) as u64;

    // Read DOS stub (bytes between DOS header and NT headers)
    let mut dos_stub = vec![0u8; (e_lfanew - 64) as usize];
    file.read_exact(&mut dos_stub)?;

    // Read NT headers (starting at e_lfanew)
    file.seek(SeekFrom::Start(e_lfanew))?;
    let mut nt_headers = vec![0u8; 248]; // Typical size of NT headers
    file.read_exact(&mut nt_headers)?;

    // Read Section Table headers (immediately following NT headers)
    let mut section_headers = Vec::new();
    file.read_to_end(&mut section_headers)?;

    // print the headers for debugging
    println!("DOS Header: {:?}", &dos_header[..]);
    println!("DOS Stub: {:?}", &dos_stub[..]);
    println!("NT Headers: {:?}", &nt_headers[..]);
    println!("Section Headers: {:?}", &section_headers[..]);
    
    Ok(PEHeaders {
        dos_header,
        dos_stub,
        nt_headers,
        section_headers,
    })
}
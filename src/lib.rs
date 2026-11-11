use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn find_footer(mut file: &File) -> Vec<u8> {
    file.seek(SeekFrom::End(-8)).unwrap();

    let mut tail = [0u8; 8];
    file.read_exact(&mut tail).unwrap();

    let magic_bytes = &tail[4..8];
    if magic_bytes != b"PAR1" {
        panic!("Not a valid parquet file!!")
    }

    let footer_length: u32 = u32::from_le_bytes(tail[0..4].try_into().unwrap());

    println!("Footer is long: {}", footer_length);

    let footer_start = -(8 + footer_length as i64);
    file.seek(SeekFrom::End(footer_start)).unwrap();

    let mut footer = vec![0u8; footer_length as usize];
    file.read_exact(&mut footer).unwrap();

    println!("File footer in bytes: {:02x?}", footer);
    footer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_footer_length() {
        let file = File::open("../simple_int32.parquet").unwrap();
        let footer = find_footer(&file);
        assert_eq!(footer.len(), 116);
        assert_eq!(footer[0], 0x15);
        assert_eq!(*footer.last().unwrap(), 0x00);
    }
}

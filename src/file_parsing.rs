//! Back to sequences: find the origin of kmers

/* std use */
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

/// Parses a file and returns a vector of Strings
/// each line in the file is a String
/// This is used to parse input / output file lists
pub fn read_file_lines<P>(file_path: P) -> Result<Vec<std::path::PathBuf>, Error>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let reader = BufReader::new(File::open(file_path)?);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(std::path::PathBuf::from(&line?));
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    /* project use */
    use super::*;

    use std::io::Write as _;

    const INPUTS: &[u8] = b"input_1.fasta
input_2.fasta
input_3.fasta
";

    #[test]
    fn inputs_file() -> std::io::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();
        let path = temp_path.join("input.lst");

        std::fs::File::create(format!("{}", path.display()))?.write_all(INPUTS)?;

        let inputs = read_file_lines(&format!("{}", path.display()))?;

        assert_eq!(
            inputs,
            vec![
                std::path::PathBuf::from("input_1.fasta"),
                std::path::PathBuf::from("input_2.fasta"),
                std::path::PathBuf::from("input_3.fasta"),
            ]
        );

        Ok(())
    }
}

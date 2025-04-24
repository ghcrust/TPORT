use std::io::{BufRead, BufReader};
use std::fs::File;
use anyhow::{Result, Context};

pub fn count_lines(br: impl BufRead) -> Result<usize> {
    let mut count = 0;
    for line in br.lines() {
        line?;
        count += 1;
    }
    Ok(count)
}

pub fn count_lines_in_path(path: &String) -> Result<usize> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file); 
    file.lines().try_fold(0, |count, line| {
        line.with_context(|| path.clone()).map(|_| count + 1)
    })
}

#[cfg(test)]
mod tests {
    use crate::count_lines;
    use std::io::{Cursor, BufReader, Read, Error, ErrorKind};
    use super::*;
    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "error"))
        }
    }
    #[test]    
    fn count_lines_fn_reeturns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err(), "no errors returned");
    }
    #[test]    
    fn count_lines_fn_counts_lines_in_input() {
        let lines = Cursor::new("one\ntwo\nthree lines");
        let got = count_lines(lines)
            .map_err(|e| {
                panic!("{e}");
            }).unwrap();
        let want = 3;
        assert_eq!(got, want, "count_lines(): want {want}, got {got:?}");
    }
    #[test]
    fn count_lines_in_path_fn_counts_lines_in_path() {
        let path = String::from("testdata/file.txt");
        let got = count_lines_in_path(&path)
            .map_err(|e| {
                panic!("{e}");
            }).unwrap();
        let want = 3;
        assert_eq!(got, want, "Want {want} got {got}");
    }
}


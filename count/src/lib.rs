use std::io::{BufRead, Result};

pub fn count_lines(br: impl BufRead) -> Result<usize> {
    /*
    br.lines().try_fold(0, |count, line| {
        line.map(|_| count + 1)
    })
     */
    let mut count = 0;
    for line in br.lines() {
        line?;
        count += 1;
    }
    Ok(count)
}


#[cfg(test)]
mod tests {
    use crate::count_lines;
    use std::io::{Error, Cursor, BufReader, Read, ErrorKind};
    
    use super::*;
    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Err(Error::new(ErrorKind::Other, "oh no"))
        }
    }
    #[test]    
    fn count_lines_fn_reeturns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err(), "no errors returned");
    }
    /*

    #[test]    
    fn count_lines_fn_counts_lines_in_input() {
        let lines = Cursor::new("one\ntwo\nthree lines");
        let got = count_lines(lines)?;
        assert_eq!(got, 3, "count_lines(): want 3, got {got:?}");
    }
    */
}

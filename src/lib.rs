// Note: add the regex and case-insensitive functionality

#[derive(Debug)]
pub struct LineResult<'a> {
    pub index: usize,
    pub line: &'a str,
}

impl<'a> PartialEq for LineResult<'a> {
    fn eq(&self, other: &LineResult) -> bool {
        self.index == other.index && self.line == other.line
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<LineResult<'a>> {
    let results = contents
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineResult {
            index: index + 1,
            line: line.trim(),
        })
        .filter(|line_result| line_result.line.contains(query))
        .map(|line_result| LineResult {
            index: line_result.index,
            line: line_result.line,
        })
        .collect::<Vec<LineResult>>();

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<LineResult<'a>> {
    let query = query.to_lowercase();

    let results = contents
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineResult {
            index: index + 1,
            line: line.trim(),
        })
        .filter(|line_result| line_result.line.to_lowercase().contains(&query))
        .map(|line_result| LineResult {
            index: line_result.index,
            line: line_result.line,
        })
        .collect::<Vec<LineResult>>();

    results
}

pub fn search_case_insensitive_ascii<'a>(query: &str, contents: &'a str) -> Vec<LineResult<'a>> {
    let query = query.to_ascii_lowercase();

    let results = contents
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineResult {
            index: index + 1,
            line: line.trim(),
        })
        .filter(|line_result| line_result.line.to_ascii_lowercase().contains(&query))
        .map(|line_result| LineResult {
            index: line_result.index,
            line: line_result.line,
        })
        .collect::<Vec<LineResult>>();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let contents = "\
        Rust.
        Safe, fast, and productive.
        C-Sharp. Maybe just two
        Duct tape.
        ";
        let query = "duct";

        assert_eq!(
            vec![LineResult {
                index: 2,
                line: "Safe, fast, and productive."
            }],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let contents = "\
        Rust.
        Safe, fast, and productive.
        C-Sharp. Maybe just two
        Trust me.
        ";
        let query = "RuSt";

        let search_matches: Vec<LineResult> = vec![
            LineResult {
                index: 1,
                line: "Rust.",
            },
            LineResult {
                index: 4,
                line: "Trust me.",
            },
        ];

        assert_eq!(search_matches, search_case_insensitive(query, contents))
    }

    #[test]
    fn case_insensitive_ascii() {
        let contents = "\
        Rust.
        Safe, fast, and productive.
        C-Sharp. Maybe just two
        Trust me.
        ";
        let query = "RuSt";

        let search_matches: Vec<LineResult> = vec![
            LineResult {
                index: 1,
                line: "Rust.",
            },
            LineResult {
                index: 4,
                line: "Trust me.",
            },
        ];

        assert_eq!(search_matches, search_case_insensitive(query, contents))
    }
}

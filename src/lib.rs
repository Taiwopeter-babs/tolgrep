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
    let mut results: Vec<LineResult> = Vec::new();

    for (index, line) in &mut contents.lines().enumerate() {
        if line.contains(query) {
            results.push(LineResult {
                index: index + 1,
                line: line.trim(),
            });
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<LineResult<'a>> {
    let mut results: Vec<LineResult> = Vec::new();
    let query = query.to_lowercase();

    for (index, line) in &mut contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push(LineResult {
                index: index + 1,
                line: line.trim(),
            });
        }
    }

    results
}

pub fn search_case_insensitive_ascii<'a>(query: &str, contents: &'a str) -> Vec<LineResult<'a>> {
    let mut results: Vec<LineResult> = Vec::new();
    let query = query.to_ascii_lowercase();

    for (index, line) in &mut contents.lines().enumerate() {
        if line.to_ascii_lowercase().contains(&query) {
            results.push(LineResult {
                index: index + 1,
                line: line.trim(),
            });
        }
    }

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

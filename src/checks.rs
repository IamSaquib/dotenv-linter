use crate::common::*;

mod duplicated_key;
mod incorrect_delimiter;
mod key_without_value;
mod leading_character;
mod lowercase_key;
mod quote_character;
mod space_character;
mod unordered_key;

// This trait is used for checks which needs to know of only a single line
trait Check {
    fn run(&mut self, line: &LineEntry) -> Option<Warning>;
    fn name(&self) -> &str;
}

// Checklist for checks which needs to know of only a single line
fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(duplicated_key::DuplicatedKeyChecker::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterChecker::default()),
        Box::new(leading_character::LeadingCharacterChecker::default()),
        Box::new(key_without_value::KeyWithoutValueChecker::default()),
        Box::new(lowercase_key::LowercaseKeyChecker::default()),
        Box::new(quote_character::QuoteCharacterChecker::default()),
        Box::new(space_character::SpaceCharacterChecker::default()),
        Box::new(unordered_key::UnorderedKeyChecker::default()),
    ]
}

pub fn run(lines: Vec<LineEntry>, skip_checks: &Vec<&str>) -> Vec<Warning> {
    let mut checks = checklist();
    checks.retain(|c| !skip_checks.contains(&c.name()));

    let mut warnings: Vec<Warning> = Vec::new();

    for line in lines {
        if line.is_empty_or_comment() {
            continue;
        }

        for ch in &mut checks {
            if let Some(warning) = ch.run(&line) {
                warnings.push(warning);
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn run_with_empty_vec_test() {
        let empty: Vec<LineEntry> = Vec::new();
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(empty, &skip_checks));
    }

    #[test]
    fn run_with_empty_line_test() {
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(""),
        };

        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn run_with_comment_line_test() {
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("# Comment"),
        };

        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn run_with_valid_line_test() {
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };

        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn run_with_invalid_line_test() {
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO"),
        };
        let warning = Warning::new(
            line.clone(),
            String::from(
                "KeyWithoutValue: The FOO key should be with a value or have an equal sign",
            ),
        );
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn skip_all_checks() {
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO"),
        };
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue"];

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn skip_one_check() {
        let line1 = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO"),
        };
        let line2 = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("1FOO"),
        };
        let warning = Warning::new(
            line1.clone(),
            String::from("LeadingCharacter: Invalid leading character detected"),
        );
        let lines: Vec<LineEntry> = vec![line1, line2];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue"];

        assert_eq!(expected, run(lines, &skip_checks));
    }
}

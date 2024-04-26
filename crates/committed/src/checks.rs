use crate::report;
use committed::Style;

pub(crate) fn check_message(
    source: report::Source<'_>,
    message: &str,
    config: &crate::config::Config,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let mut failed = false;

    failed |= check_has_message(source, message, report)?;
    if failed {
        return Ok(failed);
    }

    if config.no_wip() {
        failed |= check_wip(source, message, report)?;
    }
    if config.no_fixup() {
        failed |= check_fixup(source, message, report)?;
    }
    // Bail out due to above checks
    if failed {
        return Ok(failed);
    }

    let parsed: Option<Box<dyn Style>> = match config.style() {
        crate::config::Style::Conventional => {
            let parsed = committed::conventional::Commit::parse(message);
            match parsed {
                Ok(parsed) => Some(Box::new(parsed)),
                Err(error) => {
                    report(report::Message::error(
                        source,
                        report::InvalidCommitFormat {
                            error: anyhow::Error::new(error),
                            style: config.style(),
                        },
                    ));
                    failed = true;
                    None
                }
            }
        }
        crate::config::Style::None => {
            let parsed = committed::no_style::Commit::parse(message);
            match parsed {
                Ok(parsed) => Some(Box::new(parsed)),
                Err(error) => {
                    report(report::Message::error(
                        source,
                        report::InvalidCommitFormat {
                            error,
                            style: config.style(),
                        },
                    ));
                    failed = true;
                    None
                }
            }
        }
    };
    if let Some(parsed) = parsed {
        if config.imperative_subject() {
            failed |= check_imperative_subject(source, parsed.subject(), report)?;
        }
        if config.subject_capitalized() {
            failed |= check_capitalized_subject(source, parsed.subject(), report)?;
        }
        if config.subject_not_punctuated() {
            failed |= check_subject_not_punctuated(source, parsed.subject(), report)?;
        }

        let allowed_types: Vec<_> = config.allowed_types().collect();
        if !allowed_types.is_empty() {
            if let Some(used_type) = parsed.type_() {
                failed |= check_allowed_types(source, used_type, allowed_types, report)?;
            }
        }
    }

    if config.subject_length() != 0 {
        failed |= check_subject_length(source, message, config.subject_length(), report)?;
    }
    if config.line_length() != 0 {
        failed |= check_line_length(source, message, config.line_length(), report)?;
    }
    if config.hard_line_length() != 0 {
        failed |= check_hard_line_length(source, message, config.line_length(), report)?;
    }

    Ok(failed)
}

fn check_has_message(
    source: report::Source<'_>,
    message: &str,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    if message.trim().is_empty() {
        report(report::Message::error(source, report::EmptyCommit {}));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn check_subject_length(
    source: report::Source<'_>,
    message: &str,
    max_length: usize,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let line = message
        .split('\n')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Commit cannot be empty"))?;
    let line = line.trim_end();
    let last_space_index = line.rfind(' ').unwrap_or(0);
    let soft_line = &line[0..last_space_index];
    let count = unicode_segmentation::UnicodeSegmentation::graphemes(soft_line, true).count();
    if max_length < count {
        let count = unicode_segmentation::UnicodeSegmentation::graphemes(line, true).count();
        report(report::Message::error(
            source,
            report::SubjectTooLong {
                max_length,
                actual_length: count,
            },
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn check_line_length(
    source: report::Source<'_>,
    message: &str,
    max_length: usize,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let mut failed = false;
    for line in message.split('\n') {
        let line = line.trim_end();
        let last_space_index = line.rfind(' ').unwrap_or(0);
        let soft_line = &line[0..last_space_index];
        let count = unicode_segmentation::UnicodeSegmentation::graphemes(soft_line, true).count();
        if max_length < count {
            let count = unicode_segmentation::UnicodeSegmentation::graphemes(line, true).count();
            report(report::Message::error(
                source,
                report::LineTooLong {
                    max_length,
                    actual_length: count,
                },
            ));
            failed = true;
        }
    }
    Ok(failed)
}

pub(crate) fn check_hard_line_length(
    source: report::Source<'_>,
    message: &str,
    max_length: usize,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let mut failed = false;
    for line in message.split('\n') {
        let line = line.trim_end();
        let count = unicode_segmentation::UnicodeSegmentation::graphemes(line, true).count();
        if max_length < count {
            report(report::Message::error(
                source,
                report::LineTooLong {
                    max_length,
                    actual_length: count,
                },
            ));
            failed = true;
        }
    }
    Ok(failed)
}

pub(crate) fn check_capitalized_subject(
    source: report::Source<'_>,
    subject: &str,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let first_word = subject
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Subject cannot be empty"))?;
    if !is_capitalized(first_word)? {
        report(report::Message::error(
            source,
            report::CapitalizeSubject { first_word },
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

fn is_capitalized(word: &str) -> Result<bool, anyhow::Error> {
    let first = word
        .chars()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Subject cannot be empty"))?;
    Ok(!first.is_lowercase())
}

#[test]
fn lower_isnt_capitalized() {
    assert!(!is_capitalized("lower").unwrap());
}

#[test]
fn upper_is_capitalized() {
    assert!(is_capitalized("Upper").unwrap());
}

#[test]
fn caseless_is_capitalized() {
    assert!(is_capitalized("あ").unwrap());
}

pub(crate) fn check_subject_not_punctuated(
    source: report::Source<'_>,
    subject: &str,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let last = subject
        .chars()
        .last()
        .ok_or_else(|| anyhow::anyhow!("Subject cannot be empty"))?;
    if " .!?".contains(last) {
        report(report::Message::error(
            source,
            report::NoPunctuation { punctuation: last },
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn check_imperative_subject(
    source: report::Source<'_>,
    subject: &str,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    let first_word = subject
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Subject cannot be empty"))?;
    if !imperative::Mood::new()
        .is_imperative(first_word)
        .unwrap_or(true)
    {
        report(report::Message::error(
            source,
            report::Imperative { first_word },
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

fn check_allowed_types(
    source: report::Source<'_>,
    parsed: unicase::UniCase<&str>,
    allowed_types: Vec<&str>,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    for allowed_type in allowed_types.iter() {
        let allowed_type = unicase::UniCase::new(allowed_type);
        if allowed_type == parsed {
            return Ok(false);
        }
    }

    report(report::Message::error(
        source,
        report::DisallowedCommitType {
            used: parsed.as_ref().to_owned(),
            allowed: allowed_types.iter().map(|s| (*s).to_owned()).collect(),
        },
    ));
    Ok(true)
}

// For Gitlab's rules, see https://docs.gitlab.com/ee/user/project/merge_requests/work_in_progress_merge_requests.html
static WIP_RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(r"^(wip\b|WIP\b|\[WIP\]|Draft\b|\[Draft\]|\(Draft\))").unwrap()
});

pub(crate) fn check_wip(
    source: report::Source<'_>,
    message: &str,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    if WIP_RE.is_match(message) {
        report(report::Message::error(source, report::Wip {}));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn check_fixup(
    source: report::Source<'_>,
    message: &str,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    if message.starts_with("fixup! ") {
        report(report::Message::error(source, report::Fixup {}));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn check_merge_commit(
    source: report::Source<'_>,
    commit: &git2::Commit<'_>,
    report: report::Report,
) -> Result<bool, anyhow::Error> {
    if 1 < commit.parent_count() {
        report(report::Message::error(
            source,
            report::MergeCommitDisallowed {},
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

use crate::report;

pub fn check_message(
    source: report::Source,
    message: &str,
    config: &crate::config::Config,
    report: report::Report,
) -> Result<bool, failure::Error> {
    let mut failed = false;
    if !config.no_wip() {
        failed = failed || check_wip(source, message, report)?;
    }
    if !config.no_fixup() {
        failed = failed || check_fixup(source, message, report)?;
    }

    // Bail out due to above checks
    if failed {
        return Ok(failed);
    }

    match config.style() {
        crate::config::Style::Conventional => {
            let parsed = committed::conventional::Message::parse(message).unwrap();
            if config.imperative_subject() {
                failed = failed || check_imperative_subject(source, parsed.subject, report)?;
            }
            if config.subject_capitalized() {
                failed = failed || check_capitalized_subject(source, parsed.subject, report)?;
            }
            if config.subject_not_punctuated() {
                failed = failed || check_subject_not_punctuated(source, parsed.subject, report)?;
            }
        }
        crate::config::Style::None => {
            let parsed = committed::no_style::Message::parse(message).unwrap();
            if config.imperative_subject() {
                failed = failed || check_imperative_subject(source, parsed.raw_subject, report)?;
            }
            if config.subject_capitalized() {
                failed = failed || check_capitalized_subject(source, parsed.raw_subject, report)?;
            }
            if config.subject_not_punctuated() {
                failed =
                    failed || check_subject_not_punctuated(source, parsed.raw_subject, report)?;
            }
        }
    }
    if config.subject_length() != 0 {
        failed = failed || check_subject_length(source, message, config.subject_length(), report)?;
    }
    if config.line_length() != 0 {
        failed = failed || check_line_length(source, message, config.line_length(), report)?;
    }

    Ok(failed)
}

pub fn check_subject_length(
    source: report::Source,
    message: &str,
    max_length: usize,
    report: report::Report,
) -> Result<bool, failure::Error> {
    let subject = message
        .split('\n')
        .next()
        .ok_or_else(|| failure::Context::new("Commit cannot be empty"))?;
    let subject = subject.trim_end();
    let count = unicode_segmentation::UnicodeSegmentation::graphemes(subject, true).count();
    if max_length < count {
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

pub fn check_line_length(
    source: report::Source,
    message: &str,
    max_length: usize,
    report: report::Report,
) -> Result<bool, failure::Error> {
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

pub fn check_capitalized_subject(
    source: report::Source,
    subject: &str,
    report: report::Report,
) -> Result<bool, failure::Error> {
    let first_word = subject
        .split_whitespace()
        .next()
        .ok_or_else(|| failure::Context::new("Subject cannot be empty"))?;
    let first = first_word
        .chars()
        .next()
        .ok_or_else(|| failure::Context::new("Subject cannot be empty"))?;
    if !first.is_uppercase() {
        report(report::Message::error(
            source,
            report::CapitalizeSubject { first_word },
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn check_subject_not_punctuated(
    source: report::Source,
    subject: &str,
    report: report::Report,
) -> Result<bool, failure::Error> {
    let last = subject
        .chars()
        .last()
        .ok_or_else(|| failure::Context::new("Subject cannot be empty"))?;
    if last.is_ascii_punctuation() {
        report(report::Message::error(
            source,
            report::NoPunctuation { punctuation: last },
        ));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn check_imperative_subject(
    source: report::Source,
    subject: &str,
    report: report::Report,
) -> Result<bool, failure::Error> {
    let first_word = subject
        .split_whitespace()
        .next()
        .expect("Subject should have at least one word");
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

static WIP_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^(wip|WIP)\\b").unwrap());

pub fn check_wip(
    source: report::Source,
    message: &str,
    report: report::Report,
) -> Result<bool, failure::Error> {
    if WIP_RE.is_match(message) {
        report(report::Message::error(source, report::Wip {}));
        Ok(true)
    } else {
        Ok(false)
    }
}

static FIXUP_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^fixup! ").unwrap());

pub fn check_fixup(
    source: report::Source,
    message: &str,
    report: report::Report,
) -> Result<bool, failure::Error> {
    if FIXUP_RE.is_match(message) {
        report(report::Message::error(source, report::Fixup {}));
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn check_merge_commit(
    source: report::Source,
    commit: &git2::Commit,
    report: report::Report,
) -> Result<bool, failure::Error> {
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

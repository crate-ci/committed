# `committed` Reference

## Configuration

### Sources

Configuration is read from the following (in precedence order)

- Command line arguments
- Either
  - File specified via `--config PATH`
  - `$GIT/committed.toml`

### Config Fields

| Field                  | Argument          | Format | Description |
|------------------------|-------------------|--------|-------------|
| subject_length         | \-                | number | Number of columns the subject can occupy |
| line_length            | \-                | number | Number of columns any line can occupy, including subject |
| subject_capitalized    | \-                | bool   | Whether the subject is required to be capitalized |
| subject_not_punctuated | \-                | bool   | Prevent the subject from ending in punctuation |
| imperative_subject     | \-                | bool   | Require the subject to start with an imperative verb |
| no_fixup               | \-                | bool   | Disallow fixup commits |
| no_wip                 | \-                | bool   | Disallow WIP commits |
| style                  | \-                | none, [conventional] | Commit style convention |
| merge_commit           | --no-merge-commit | \-     | Disallow merge commits |

[conventional]: https://www.conventionalcommits.org/

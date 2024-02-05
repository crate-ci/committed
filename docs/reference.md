# `committed` Reference

## Specifying commits

Without any commits specified, `committed` will detect if something is being
piped in on `stdin` and use that, otherwise it will check `HEAD`.

### Commits

```bash
committed HEAD
```

### Commit Ranges

```bash
committed master..HEAD
```

- The range excludes the start commit
- This will Do The Right Thing even when `master` is ahead of when you
  branched.  `committed` will look for the merge-base between the range end
  points.

### Commit Files and `stdin`

This is useful for editor integration:

```bash
committed --commit-file <path>
# Or for stdin
committed --commit-file -
```

- This will not run all verification checks, like looking for merge commits.

## Configuration

### Sources

Configuration is read from the following (in precedence order)

- Command line arguments
- Either
  - File specified via `--config PATH`
  - `<git repo directory>/committed.toml`

### Config Fields

| Field                  | Argument          | Format               | Default                                             | Description                                                                                                           |
|------------------------|-------------------|----------------------|-----------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| ignore_author_re       | \-                | regx                 | (none)                                              | Authors to ignore the commits for. Generally used with bots out of your control.                                      |
| subject_length         | \-                | number               | 50                                                  | Number of columns the subject can occupy                                                                              |
| line_length            | \-                | number               | 72                                                  | Number of columns any line with a break can occupy, including subject                                                 |
| hard_line_length       | \-                | number               | 0 (none)                                            | Max number of columns any line can occupy.                                                                            |
| subject_capitalized    | \-                | bool                 | true                                                | Whether the subject is required to be capitalized                                                                     |
| subject_not_punctuated | \-                | bool                 | true                                                | Prevent the subject from ending in punctuation                                                                        |
| imperative_subject     | \-                | bool                 | true                                                | Require the subject to start with an imperative verb                                                                  |
| no_fixup               | \-                | bool                 | true                                                | Disallow fixup commits                                                                                                |
| no_wip                 | \-                | bool                 | true                                                | Disallow WIP commits                                                                                                  |
| style                  | \-                | none, [conventional] | none                                                | Commit style convention                                                                                               |
| allowed_types          | \-                | list of strings      | fix, feat, chore, docs, style, refactor, perf, test | _(Conventional)_ Accepted commit types                                                                                |
| merge_commit           | --no-merge-commit | \-                   | true                                                | Disallow merge commits. Argument is recommended over config file since there are times when merge-commits are wanted. |

[conventional]: https://www.conventionalcommits.org/

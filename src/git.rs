pub struct RevSpec<'r> {
    from: git2::Commit<'r>,
    to: Option<git2::Commit<'r>>,
}

impl<'r> RevSpec<'r> {
    pub fn parse(repo: &'r git2::Repository, revspec: &str) -> Result<Self, failure::Error> {
        let commits = repo.revparse(revspec)?;
        let from = commits.from().unwrap().as_commit().unwrap().clone();
        let to = commits.to().map(|o| o.as_commit().unwrap().clone());
        let from = if let Some(ref to) = to.as_ref() {
            let merged_from_id = repo.merge_base(from.id(), to.id())?;
            if merged_from_id != from.id() {
                log::debug!(
                    "from/to for revspec {} are on different branches, relying on common parent {}",
                    revspec,
                    merged_from_id
                );
            }
            repo.find_commit(merged_from_id)?
        } else {
            from
        };
        Ok(Self { from, to })
    }

    pub fn iter(&self) -> RevSpecIterator {
        if let Some(ref to) = self.to.as_ref() {
            if self.from.id() == to.id() {
                RevSpecIterator {
                    revspec: self,
                    start: None,
                    parents: None,
                }
            } else {
                RevSpecIterator {
                    revspec: self,
                    start: Some(to),
                    parents: Some(to.parents()),
                }
            }
        } else {
            RevSpecIterator {
                revspec: self,
                start: Some(&self.from),
                parents: None,
            }
        }
    }
}

pub struct RevSpecIterator<'r> {
    revspec: &'r RevSpec<'r>,
    start: Option<&'r git2::Commit<'r>>,
    parents: Option<git2::Parents<'r, 'r>>,
}

impl<'r> Iterator for RevSpecIterator<'r> {
    type Item = git2::Commit<'r>;

    fn next(&mut self) -> Option<git2::Commit<'r>> {
        if let Some(start) = self.start {
            self.start = None;
            Some(start.clone())
        } else if let Some(parents) = self.parents.as_mut() {
            if let Some(parent) = parents.next() {
                if parent.id() != self.revspec.from.id() {
                    Some(parent)
                } else {
                    self.parents = None;
                    None
                }
            } else {
                self.parents = None;
                None
            }
        } else {
            None
        }
    }
}

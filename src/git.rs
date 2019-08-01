pub struct RevSpec<'r> {
    from: git2::Commit<'r>,
    to: git2::Commit<'r>,
}

impl<'r> RevSpec<'r> {
    pub fn parse(repo: &'r git2::Repository, revspec: &str) -> Result<Self, failure::Error> {
        let commits = repo.revparse(revspec)?;
        let from = commits.from().unwrap().as_commit().unwrap().clone();
        let to = commits
            .to()
            .map(|o| o.as_commit().unwrap().clone())
            .unwrap_or_else(|| {
                let id = repo.refname_to_id("HEAD").unwrap();
                repo.find_commit(id).unwrap()
            });
        let merged_from_id = repo.merge_base(from.id(), to.id())?;
        if merged_from_id != from.id() {
            log::debug!(
                "from/to for revspec {} are on different branches, relying on common parent {}",
                revspec,
                merged_from_id
            );
        }
        let merged_from = repo.find_commit(merged_from_id)?;
        Ok(Self {
            from: merged_from,
            to,
        })
    }

    pub fn iter(&self) -> RevSpecIterator {
        RevSpecIterator {
            revspec: &self,
            needs_to: true,
            needs_parents: true,
            parents: self.to.parents(),
        }
    }
}

pub struct RevSpecIterator<'r> {
    revspec: &'r RevSpec<'r>,
    needs_to: bool,
    needs_parents: bool,
    parents: git2::Parents<'r, 'r>,
}

impl<'r> Iterator for RevSpecIterator<'r> {
    type Item = git2::Commit<'r>;

    fn next(&mut self) -> Option<git2::Commit<'r>> {
        if self.needs_to {
            if self.revspec.to.id() == self.revspec.from.id() {
                return None;
            } else {
                self.needs_to = false;
                return Some(self.revspec.to.clone());
            }
        } else if self.needs_parents {
            if let Some(parent) = self.parents.next() {
                if parent.id() == self.revspec.from.id() {
                    self.needs_parents = false;
                } else {
                    return Some(parent);
                }
            }
        }
        None
    }
}

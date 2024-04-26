pub(crate) struct RevSpec<'r> {
    repo: &'r git2::Repository,
    from: git2::Commit<'r>,
    to: Option<git2::Commit<'r>>,
}

impl<'r> RevSpec<'r> {
    pub(crate) fn parse(repo: &'r git2::Repository, revspec: &str) -> Result<Self, anyhow::Error> {
        let commits = repo.revparse(revspec)?;
        let from = commits.from().unwrap().as_commit().unwrap().clone();
        let to = commits.to().map(|o| o.as_commit().unwrap().clone());
        let from = if let Some(to) = to.as_ref() {
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
        Ok(Self { repo, from, to })
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = git2::Commit<'r>> {
        if let Some(to) = self.to.as_ref() {
            let range = format!("{}..{}", self.from.id(), to.id());
            let mut revwalk = self.repo.revwalk().unwrap();
            revwalk.push_range(&range).unwrap();
            let revwalk = RevWalkIterator {
                repo: self.repo,
                revwalk,
            };
            itertools::Either::Left(revwalk)
        } else {
            itertools::Either::Right(Some(self.from.clone()).into_iter())
        }
    }
}

struct RevWalkIterator<'r> {
    repo: &'r git2::Repository,
    revwalk: git2::Revwalk<'r>,
}

impl<'r> Iterator for RevWalkIterator<'r> {
    type Item = git2::Commit<'r>;

    fn next(&mut self) -> Option<git2::Commit<'r>> {
        if let Some(next) = self.revwalk.next() {
            next.ok().and_then(|o| self.repo.find_commit(o).ok())
        } else {
            None
        }
    }
}

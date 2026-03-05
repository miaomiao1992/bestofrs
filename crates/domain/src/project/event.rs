use crate::Repo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectCreated {
    pub repo: Repo,
}

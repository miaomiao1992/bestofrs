use domain::{ProjectCreated, Repo};
use crate::app_error::AppResult;
use crate::repo::RepoCommandHandler;

#[derive(Clone)]
pub struct ProjectEventHandler {
    repo_command: RepoCommandHandler,
}

impl ProjectEventHandler {
    pub fn new(repo_command: RepoCommandHandler) -> Self {
        Self { repo_command }
    }

    pub async fn handle_project_created(&self, event: &ProjectCreated) -> AppResult<()> {
        self.handle_projects_created(std::slice::from_ref(event)).await
    }

    pub async fn handle_projects_created(&self, events: &[ProjectCreated]) -> AppResult<()> {
        if events.is_empty() {
            return Ok(());
        }
        let repos = events
            .iter()
            .map(|event| event.repo.clone())
            .collect::<Vec<Repo>>();
        self.repo_command.upsert_many(&repos).await
    }
}

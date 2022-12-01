use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    executor::executor::DynExecutor,
    git::{gitea::DynGiteaProvider, DynGitProvider},
    schema::models::{Action, Gitea},
};

pub struct GiteaSelector {
    gitea_provider: DynGiteaProvider,
    git_provider: DynGitProvider,
    executor: DynExecutor,
}

impl GiteaSelector {
    pub fn new(
        gitea_provider: DynGiteaProvider,
        git_provider: DynGitProvider,
        executor: DynExecutor,
    ) -> Self {
        Self {
            gitea_provider,
            git_provider,
            executor,
        }
    }

    pub async fn run(
        &self,
        git: &Gitea,
        action_path: &PathBuf,
        action: &Action,
        dryrun: bool,
    ) -> eyre::Result<()> {
        tracing::info!("fetching repos");
        for repo in &git.repositories {
            let gp = self.gitea_provider.clone();
            let (path, repo) = gp.clone_from_qualified(repo).await?;
            let repo = Arc::new(Mutex::new(repo));

            if let Some(push) = &git.push {
                self.git_provider
                    .create_branch(repo.clone(), &push.pull_request.name)
                    .await?;
            }

            self.executor.execute(&path, action_path, action).await?;

            if dryrun {
                continue;
            }

            if let Some(push) = &git.push {
                self.git_provider
                    .push_branch(repo, &push.pull_request.name)
                    .await?;
            }
        }

        Ok(())
    }
}

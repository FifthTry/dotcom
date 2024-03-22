pub struct Reconfigure {
    github_org: String,
    github_repo_name: String,
    github_repo_branch: String,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for Reconfigure {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft2::errors::ToActionError;
        pub use ft_sdk::JsonBodyExt;

        let body = c.in_.req.json_body()?;
        let github_org_input = body.field::<String>("github-org")?.unwrap_or_default();
        let github_org = ft2::validation::validate_github_org(github_org_input.as_str())
            .map_err(|e| e.to_action_error())?;

        let github_repo_name_input = body
            .field::<String>("github-repo-name")?
            .unwrap_or_default();
        let github_repo_name =
            ft2::validation::validate_github_repo_name(github_repo_name_input.as_str())
                .map_err(|e| e.to_action_error())?;

        let github_repo_branch_input = body
            .field::<String>("github-repo-branch")?
            .unwrap_or_default();
        let github_repo_branch =
            ft2::validation::validate_github_repo_branch(github_repo_branch_input.as_str())
                .map_err(|e| e.to_action_error())?;

        Ok(Reconfigure {
            github_org,
            github_repo_name,
            github_repo_branch,
        })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_gh_oidc_repo_rule;

        // todo: need configure repo history to make rate limit checks per organization
        let github_repo = format!(
            "{}/{}",
            self.github_org.as_str(),
            self.github_repo_name.as_str()
        );

        // Todo: In future we'll give facility to add more GhOidcRepoRule.
        //       So we'll remove unique constraint on site_id and this function can be insertable, not AsChangeSet
        // Updating gh-oidc entry
        c.conn.transaction(|conn| {
            match diesel::update(
                ft_gh_oidc_repo_rule::table.filter(ft_gh_oidc_repo_rule::site_id.eq(c.site_data.id)),
            ).set((
                    ft_gh_oidc_repo_rule::gh_repo.eq(github_repo.as_str()),
                    ft_gh_oidc_repo_rule::gh_branch.eq(self.github_repo_branch.as_str()),
                    ft_gh_oidc_repo_rule::updated_at.eq(c.in_.now),
                ))
                .execute(conn) {
                Ok(_) => (),
                Err(e) => return Err(ft_common::ActionError::Diesel(e)),
            };

            c.site_data.update_updated_at(conn, c.in_.now)?;
            Ok(())
        })?;

        let github_url = ft_common::urls::github_url(c.site_data.slug.as_str());
        Ok(ft_sdk::ActionOutput::Redirect(github_url))
    }
}

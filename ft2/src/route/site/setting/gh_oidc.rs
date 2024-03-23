#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GithubOidc {
    site_domain: String,
    site_slug: String,
    account: String,
    repo: String,
    branch: String,
    is_configured: bool,
    configure_url: String,
}

impl ft_sdk::Page<ft2::route::Site, ft_common::ActionError> for GithubOidc {
    fn page(i: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::{ft_gh_oidc_repo_rule, ft_site};

        // todo: need access history to perform org rate limit checks
        match ft_site::table
            .filter(ft_site::slug.eq(&i.site_data.slug))
            .inner_join(ft_gh_oidc_repo_rule::table)
            .select((
                ft_gh_oidc_repo_rule::gh_repo,
                ft_gh_oidc_repo_rule::gh_branch,
            ))
            .first::<(String, String)>(&mut i.conn)
        {
            Err(diesel::result::Error::NotFound) => Ok(GithubOidc {
                is_configured: false,
                site_slug: i.site_data.slug.to_string(),
                account: "".to_string(),
                repo: "".to_string(),
                site_domain: i.site_data.domain.clone(),
                configure_url: ft2::urls::github_configure_form_url(
                    i.site_data.slug.as_str(),
                ),
                branch: "main".to_string(),
            }),
            Err(e) => Err(e.into()),
            Ok((repo, branch)) => {
                let (account, repo) = repo.split_once('/').unwrap();
                Ok(GithubOidc {
                    site_slug: i.site_data.slug.to_string(),
                    site_domain: i.site_data.domain.clone(),
                    account: account.to_string(),
                    repo: repo.to_string(),
                    branch: branch.to_string(),
                    is_configured: true,
                    configure_url: ft2::urls::github_configure_form_url(
                        i.site_data.slug.as_str(),
                    ),
                })
            }
        }
    }
}

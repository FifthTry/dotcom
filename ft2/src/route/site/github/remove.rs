pub struct Remove {}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for Remove {
    fn validate(_c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        // No validation
        Ok(Remove {})
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_gh_oidc_repo_rule;

        c.conn.transaction(|conn| {
            match diesel::delete(ft_gh_oidc_repo_rule::table)
                .filter(ft_gh_oidc_repo_rule::site_id.eq(c.site_data.id))
                .execute(conn) {
                Ok(_) => (),
                Err(e) => return Err(ft_common::ActionError::Diesel(e)),
            };

            c.site_data.update_updated_at(conn, c.in_.now)?;
            Ok(())
        })?;

        Ok(ft_sdk::ActionOutput::Reload)
    }
}

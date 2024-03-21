pub struct Delete {
    token_id: i64,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for Delete {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        // No validation
        let body = c.in_.req.json_body()?;
        let token_id = body.field::<i64>("token-id")?.unwrap_or_default();

        Ok(Delete { token_id })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        pub use ft2::errors::ToActionError;
        use ft_common::prelude::*;
        use ft_common::schema::ft_site_token;

        c.conn.transaction(|conn| {
            match diesel::delete(ft_site_token::table)
                .filter(ft_site_token::id.eq(self.token_id))
                .filter(ft_site_token::site_id.eq(c.site_data.id))
                .execute(conn)
            {
                Ok(1) => (),
                Ok(v) => {
                    tracing::warn!("Deleted {} tokens instead of 1", v);
                }
                Err(diesel::NotFound) => {
                    // Validation returns msg: unknown-token
                    return Err(ft2::errors::DeleteTokenError::NotFound.to_action_error());
                }
                Err(e) => return Err(e.into()),
            };

            c.site_data.update_updated_at(conn, c.in_.now)?;
            Ok(())
        })?;

        Ok(ft_sdk::ActionOutput::Reload)
    }
}

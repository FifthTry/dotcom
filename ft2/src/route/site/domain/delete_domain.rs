pub struct DeleteDomain {
    domain: String,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for DeleteDomain {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        // No validation required
        let domain: String = c.in_.req.json_body()?.field("domain")?.unwrap_or_default();
        Ok(DeleteDomain { domain })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::errors::ToActionError;
        use ft_common::prelude::*;
        use ft_common::schema::ft_domain;

        if self.domain.ends_with(".fifthtry.site") {
            // Validation returns msg: cannot-delete-fifthtry-domain
            return Err(
                ft_common::errors::DeleteDomainError::CannotDeleteFifthTryDomain.to_action_error(),
            );
        }

        if c.site_data.domain == self.domain {
            // Validation returns msg: cannot-delete-primary-domain
            return Err(
                ft_common::errors::DeleteDomainError::CannotDeletePrimaryDomain.to_action_error(),
            );
        }

        // Deleting domain
        match diesel::delete(ft_domain::table)
            .filter(ft_domain::domain.eq(self.domain.clone()))
            .filter(ft_domain::site_id.eq(c.site_data.id))
            .execute(&mut c.conn)
        {
            Ok(1) => (),
            Ok(v) => {
                tracing::warn!("Deleted {} domains instead of 1", v);
            }
            Err(diesel::NotFound) => {
                // Validation returns msg: unknown-site
                return Err(ft_common::errors::DeleteDomainError::DomainNotFound.to_action_error());
            }
            Err(e) => return Err(e.into()),
        };

        c.site_data.update_updated_at(&mut c.conn, c.in_.now)?;
        Ok(ft_sdk::ActionOutput::Reload)
    }
}

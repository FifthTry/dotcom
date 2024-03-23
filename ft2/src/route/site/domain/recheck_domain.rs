pub struct RecheckDomain {
    domain: String,
}

impl ft_sdk::Action<ft2::route::Site, ft2::ActionError> for RecheckDomain {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft2::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        // No validation
        let domain: String = c.in_.req.json_body()?.field("domain")?.unwrap_or_default();
        Ok(RecheckDomain { domain })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft2::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_domain;

        // Updating dns and ssl check schedule and status
        c.conn.transaction(|conn| {
            match diesel::update(ft_domain::table)
                .filter(ft_domain::site_id.eq(c.site_data.id))
                .filter(ft_domain::domain.eq(self.domain.as_str()))
                .set((
                    ft_domain::ssl_check_scheduled_at.eq(c.in_.now),
                    ft_domain::dns_check_scheduled_at.eq(c.in_.now),
                    ft_domain::dns_status.eq(ft_common::DnsStatus::PendingNew.as_str()),
                ))
                .execute(conn) {
                Ok(_) => (),
                Err(e) => return Err(ft2::ActionError::Diesel(e)),
            };

            c.site_data.update_updated_at(conn, c.in_.now)?;
            Ok(())
        })?;

        Ok(ft_sdk::ActionOutput::Reload)
    }
}

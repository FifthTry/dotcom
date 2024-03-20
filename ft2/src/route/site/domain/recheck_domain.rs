pub struct RecheckDomain {
    domain: String,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for RecheckDomain {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        // No validation required
        let domain: String = c.in_.req.json_body()?.field("domain")?.unwrap_or_default();
        Ok(RecheckDomain { domain })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_domain;

        // Updating dns and ssl check schedule and status
        diesel::update(ft_domain::table)
            .filter(ft_domain::site_id.eq(c.site_data.id))
            .filter(ft_domain::domain.eq(self.domain.as_str()))
            .set((
                ft_domain::ssl_check_scheduled_at.eq(c.in_.now),
                ft_domain::dns_check_scheduled_at.eq(c.in_.now),
                ft_domain::dns_status.eq(ft_common::DnsStatus::PendingNew.as_str()),
            ))
            .execute(&mut c.conn)?;

        c.site_data.update_updated_at(&mut c.conn, c.in_.now)?;
        Ok(ft_sdk::ActionOutput::Reload)
    }
}

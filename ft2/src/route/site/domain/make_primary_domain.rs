pub struct MakePrimaryDomain {
    domain: String,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for MakePrimaryDomain {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        // No validation required
        let domain: String = c.in_.req.json_body()?.field("domain")?.unwrap_or_default();
        Ok(MakePrimaryDomain { domain })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::errors::ToActionError;
        use ft_common::prelude::*;
        use ft_common::schema::{ft_domain, ft_site};

        if !self.domain.ends_with(".fifthtry.site") {
            let (domain_ssl_status, domain_dns_status): (String, String) = match ft_domain::table
                .filter(ft_domain::domain.eq(self.domain.as_str()))
                .select((ft_domain::ssl_status, ft_domain::dns_status))
                .first::<(String, String)>(&mut c.conn)
            {
                Ok((ssl_status, dns_status)) => (ssl_status, dns_status),
                Err(diesel::NotFound) => {
                    // Validation returns msg: domain-is-present
                    return Err(
                        ft_common::errors::MakeDomainPrimaryError::DomainNotFound.to_action_error()
                    );
                }
                Err(e) => return Err(e.into()),
            };

            if !domain_ssl_status.eq(ft_common::SslStatus::Verified.as_str())
                || !domain_dns_status.eq(ft_common::DnsStatus::Verified.as_str())
            {
                // Validation returns msg: domain-is-verified
                return Err(
                    ft_common::errors::MakeDomainPrimaryError::DomainNotVerified.to_action_error()
                );
            }
        }

        // todo: need primary domain change history to add rate limit error checks
        diesel::update(ft_site::table)
            .filter(ft_site::id.eq(c.site_data.id))
            .set((
                ft_site::domain.eq(self.domain.as_str()),
                ft_site::updated_at.eq(c.in_.now),
            ))
            .execute(&mut c.conn)?;

        c.site_data.update_updated_at(&mut c.conn, c.in_.now)?;
        Ok(ft_sdk::ActionOutput::Reload)
    }
}

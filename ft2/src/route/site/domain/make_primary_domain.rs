pub struct MakePrimaryDomain {
    domain: String,
}

impl ft_sdk::Action<ft2::route::Site, ft2::ActionError> for MakePrimaryDomain {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft2::ActionError> {
        pub use ft_sdk::JsonBodyExt;
        pub use ft2::errors::ToActionError;
        use ft_common::prelude::*;
        use ft_common::schema::ft_domain;

        let domain: String = c.in_.req.json_body()?.field("domain")?.unwrap_or_default();

        if !domain.ends_with(".fifthtry.site") {
            let (domain_ssl_status, domain_dns_status): (String, String) = match ft_domain::table
                .filter(ft_domain::domain.eq(domain.as_str()))
                .select((ft_domain::ssl_status, ft_domain::dns_status))
                .first::<(String, String)>(&mut c.conn)
            {
                Ok((ssl_status, dns_status)) => (ssl_status, dns_status),
                Err(diesel::NotFound) => {
                    // Validation returns msg: domain-is-present
                    return Err(
                        ft2::errors::MakePrimaryDomainError::DomainNotFound.to_action_error()
                    );
                }
                Err(e) => return Err(e.into()),
            };

            if !domain_ssl_status.eq(ft_common::SslStatus::Verified.as_str())
                || !domain_dns_status.eq(ft_common::DnsStatus::Verified.as_str())
            {
                // Validation returns msg: domain-is-verified
                return Err(
                    ft2::errors::MakePrimaryDomainError::DomainNotVerified.to_action_error()
                );
            }
        }

        Ok(MakePrimaryDomain { domain })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft2::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_site;

        diesel::update(ft_site::table)
            .filter(ft_site::id.eq(c.site_data.id))
            .set((
                ft_site::domain.eq(self.domain.as_str()),
                ft_site::updated_at.eq(c.in_.now),
            ))
            .execute(&mut c.conn)
            .map_err(ft2::ActionError::Diesel)?;

        Ok(ft_sdk::ActionOutput::Reload)
    }
}

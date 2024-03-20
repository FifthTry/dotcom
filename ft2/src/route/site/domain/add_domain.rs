pub struct AddDomain {
    pub domain: String,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for AddDomain {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft_sdk::JsonBodyExt;

        let domain_input: String = c.in_.req.json_body()?.field("domain")?.unwrap_or_default();
        let domain = crate::validation::validate_domain(domain_input.as_str())?;

        Ok(AddDomain { domain })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::errors::ToActionError;
        use ft_common::prelude::*;
        use ft_common::schema::ft_domain;

        // todo: make this transaction later
        // transaction is not implemented yet for PgConnection
        match diesel::insert_into(ft_domain::table)
            .values(&ft_common::Domain {
                site_id: c.site_data.id,
                domain: self.domain.to_string(),

                dns_status: ft_common::DnsStatus::PendingNew.as_str(),
                dns_attempts: 0,
                dns_check_scheduled_at: c.in_.now,

                ssl_status: ft_common::SslStatus::PendingNew.as_str(),
                ssl_http_token: None,
                ssl_http_proof: None,
                ssl_check_scheduled_at: None,
                ssl_certificate_issued_at: None,
                ssl_certificate_pem: None,
                ssl_encrypted_private_key_pem: None,

                created_at: c.in_.now,
                updated_at: c.in_.now,
            })
            .execute(&mut c.conn)
        {
            Ok(_) => (),
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => {
                // Validation returns msg: unique-domain
                return Err(ft_common::errors::AddDomainError::DomainAlreadyExists(
                    self.domain.clone(),
                )
                .to_action_error());
            }
            Err(e) => return Err(e.into()),
        };

        c.site_data.update_updated_at(&mut c.conn, c.in_.now)?;
        Ok(ft_sdk::ActionOutput::Reload)
    }
}

pub struct Create {
    about: String,
    can_read: bool,
    can_write: bool,
}

impl ft_sdk::Action<ft2::route::Site, ft_common::ActionError> for Create {
    fn validate(c: &mut ft2::route::Site) -> Result<Self, ft_common::ActionError> {
        pub use ft_common::errors::ToActionError;
        pub use ft_sdk::JsonBodyExt;

        let body = c.in_.req.json_body()?;
        let about_input = body.field::<String>("about")?.unwrap_or_default();
        let about = crate::validation::validate_token_about(about_input.as_str())
            .map_err(|e| e.to_action_error())?;

        let can_read = body.field::<bool>("can-read")?.unwrap_or_default();
        let can_write = body.field::<bool>("can-write")?.unwrap_or_default();

        Ok(Create {
            about,
            can_read,
            can_write,
        })
    }

    fn action(
        &self,
        c: &mut ft2::route::Site,
    ) -> Result<ft_sdk::ActionOutput, ft_common::ActionError> {
        use ft_common::prelude::*;
        use ft_common::schema::ft_site_token;

        // Query the database to count domains inserted in the last one minute for the
        // specified `org_id`.
        let site_tokens_created_in_last_minute = ft_site_token::table
            .filter(ft_site_token::site_id.eq(c.site_data.id))
            .filter(
                ft_site_token::created_at.gt(c.in_.now - chrono::Duration::try_minutes(1).unwrap()),
            )
            .select(count_star())
            .first::<i64>(&mut c.conn)?;

        if site_tokens_created_in_last_minute >= 5 {
            // Validation returns msg: add-domain-org-rate-limit-reached
            return Err(ft_common::ActionError::OrgRateLimitError(
                "We only create max 5 tokens per minute. Please try after some time.".to_string(),
            ));
        }

        diesel::insert_into(ft_site_token::table)
            .values(&ft_common::SiteToken {
                about: self.about.clone(),
                token: generate_token(),
                can_read: self.can_read,
                can_write: self.can_write,
                site_id: c.site_data.id,

                created_at: c.in_.now,
                updated_at: c.in_.now,
                last_used_at: None,
                created_by: c.ud.user_id,
            })
            .execute(&mut c.conn)?;

        c.site_data.update_updated_at(&mut c.conn, c.in_.now)?;
        Ok(ft_sdk::ActionOutput::Reload)
    }
}

// todo: fix this - uuid not supported default for wasm-unknown-unknown target
// pub fn generate_token() -> String {
//     uuid::Uuid::new_v4().to_string()
// }

// todo: remove this when above is fixed
pub fn generate_token() -> String {
    "token".to_string()
}

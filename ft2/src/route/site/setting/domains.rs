// This struct is equivalent to the record we are expecting
// inside sites/domains.ftd

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Domain {
    pub domain: String,
    pub is_primary: bool,
    pub status: String, // todo: convert to enum
}

#[derive(serde::Serialize, Debug)]
#[serde(transparent)]
pub struct Domains {
    domains: Vec<Domain>,
}

impl ft_sdk::Page<ft2::route::Site, ft2::ActionError> for Domains {
    fn page(i: &mut ft2::route::Site) -> Result<Self, ft2::ActionError> {
        use ft_common::{prelude::*, schema::ft_domain};

        // todo: need access history to perform org rate limit checks

        let fetched_domains: Vec<DomainQueryData> = ft_domain::table
            .filter(ft_domain::site_id.eq(i.site_data.id))
            .select(DomainQueryData::as_select())
            .load(&mut i.conn)?;

        let mut domains = vec![];

        let fifthtry_site_domain = format!("{}.fifthtry.site", i.site_data.slug);
        domains.push(Domain {
            domain: fifthtry_site_domain.clone(),
            is_primary: fifthtry_site_domain == i.site_data.domain,
            status: "verified".to_string(),
        });

        for domain in fetched_domains {
            domains.push(Domain {
                domain: domain.domain.clone(),
                is_primary: domain.domain == i.site_data.domain,
                status: match (domain.dns_status.as_str(), domain.ssl_status.as_str()) {
                    ("verified", "verified") => "verified",
                    ("failed", _) => "failed",
                    (_, "failed") => "failed",
                    _ => "pending",
                }
                .to_string(),
            })
        }

        Ok(Domains { domains })
    }
}

#[derive(Debug, diesel::Selectable, diesel::Queryable)]
#[diesel(table_name = ft_common::schema::ft_domain)]
struct DomainQueryData {
    domain: String,
    dns_status: String,
    ssl_status: String,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_document_history)]
pub struct DocumentHistory {
    pub path: String,
    pub diff: Option<String>,
    pub is_deleted: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub editor_id: i64,
    pub site_id: i64,
    pub tejar_content_id: Option<i64>,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_document)]
pub struct Document {
    pub path: String,
    pub is_public: bool,
    pub is_ftd: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub site_id: i64,
    pub tejar_content_id: Option<i64>,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_tejar_content)]
pub struct TejarContent {
    pub s3_tejar_file_offset: i32,
    pub s3_tejar_file_size: i32,
    pub sha256_hash: String,
    pub file_id: i64,
    pub shared_count: i32,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_site_token)]
pub struct SiteToken {
    pub about: String,
    pub token: String,
    pub can_read: bool,
    pub can_write: bool,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: i64,
    pub site_id: i64,
}


#[derive(diesel::Insertable, diesel::AsChangeset)]
#[diesel(table_name = ft_common::schema::ft_gh_oidc_repo_rule)]
pub struct GhOidcRepoRule {
    pub gh_repo: String,
    pub gh_branch: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub site_id: i64,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_site)]
pub struct Site {
    pub name: String,
    pub slug: String,
    pub is_static: bool,
    pub is_public: bool,
    pub is_editable: bool,
    pub is_package: bool,
    pub domain: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub org_id: Option<i64>,
    pub created_by: i64,
}


#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_domain)]
pub struct Domain {
    pub domain: String,
    pub dns_status: &'static str,
    // todo: how to use our enum here?
    pub dns_attempts: i32,
    pub dns_check_scheduled_at: chrono::DateTime<chrono::Utc>,
    pub ssl_status: &'static str,
    // todo: how to use our enum here?
    pub ssl_http_token: Option<String>,
    pub ssl_http_proof: Option<Vec<u8>>,
    pub ssl_check_scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ssl_certificate_issued_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ssl_certificate_pem: Option<String>,
    pub ssl_encrypted_private_key_pem: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub site_id: i64,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_org)]
pub struct Org {
    pub name: String,
    pub slug: String,
    pub owner_id: i64,
    pub plan_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = ft_common::schema::ft_org_member)]
pub struct OrgMember {
    pub role: String,
    // Todo: Create enum for role
    pub org_id: i64,
    pub user_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

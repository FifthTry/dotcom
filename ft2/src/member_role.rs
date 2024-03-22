// ROLE_ADMIN = "admin"
// ROLE_GUEST = "guest"
// ROLE_MANAGER = "manager"
// ROLE_MEMBER = "member"

#[derive(PartialEq, Debug)]
pub enum MemberRole {
    // super-user, can do anything
    Admin,
    // can manage users, add remove sites, update domains etc
    Manager,
    // can view and update site content
    Member,
    // can view site content. this is usually people you invite to your team
    // maybe we need to think more about role etc.
    Guest,
}

impl MemberRole {
    pub fn has_manage_permissions(&self) -> bool {
        matches!(self, MemberRole::Admin | MemberRole::Manager)
    }
}

impl From<&str> for MemberRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => MemberRole::Admin,
            "manager" => MemberRole::Manager,
            "member" => MemberRole::Member,
            "guest" => MemberRole::Guest,
            _ => unreachable!("invalid role"),
        }
    }
}

impl MemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemberRole::Admin => "admin",
            MemberRole::Manager => "manager",
            MemberRole::Member => "member",
            MemberRole::Guest => "guest",
        }
    }
}

pub fn route(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft2::route::*;
    use ft_sdk::Layout;

    match r.uri().path() {
        // site
        "/site/info/" => Site::page::<site::Info>(r),

        // site settings
        "/site/setting/domains/" => Site::page::<site::setting::Domains>(r),
        "/site/setting/gh-oidc/" => Site::page::<site::setting::GithubOidc>(r),
        "/site/setting/tokens/" => Site::page::<site::setting::Tokens>(r),
        "/ft2/site/delete/" => todo!(),

        // editor stuff
        "/site/editor/" => Site::page::<site::Editor>(r),
        "/ft2/site/get-content/" => Site::action::<site::GetContent>(r),

        // public
        "/public/user-data/" => Public::page::<public::UserData>(r),
        "/user/dashboard/" => Public::page::<public::UserDashboard>(r),

        // org
        "/org/dashboard/" => Org::page::<public::UserDashboard>(r),

        // myself
        "/ft2/myself/create-site/" => MySelf::action::<myself::CreateSite>(r),

        t => ft_sdk::not_found!("no route for {t}"),
    }
}

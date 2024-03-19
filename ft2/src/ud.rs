#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserData {
    pub user_id: i64,
    pub username: String,
    pub name: String,
    pub notifications: usize,
    pub is_logged_in: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum GetUDError {
    #[error("user not found")]
    UserNotFound,

    #[error("diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("sdk error: {0}")]
    SDK(#[from] ft_sdk::Error),
}

pub(crate) fn get_optional_ud(
    conn: &mut ft_sdk::PgConnection,
    in_: &ft_sdk::In,
) -> Result<Option<UserData>, GetUDError> {
    // ensure ud exists in ft-db, every time this function is called, or we can assume, but
    // how do we find out the user id then, as most people need user id to do anything.
    // we use the same user id from fastn's auth in ft_user

    // get fastn ud from cookie.
    ft_sdk::println!("get_optional_ud");
    let fud = match in_.ud {
        Some(ref v) => v,
        None => {
            ft_sdk::println!("user not logged in");
            return Ok(None);
        }
    };

    ft_sdk::println!("User data: {fud:?}");

    let user_id = get_or_create_user(conn, fud, &in_.now)
        .inspect_err(|e| ft_sdk::println!("create_user returned {e:?}"))?;

    Ok(Some(UserData {
        user_id,
        username: fud.username.to_string(),
        name: fud.name.to_string(),
        notifications: 0, // todo: fix this
        is_logged_in: true,
    }))
}

fn get_or_create_user(
    conn: &mut ft_sdk::PgConnection,
    user_data: &ft_sdk::UserData,
    now: &chrono::DateTime<chrono::Utc>,
) -> Result<i64, GetUDError> {
    use ft_common::{prelude::*, schema::ft_user};

    //    ft_user (id) {
    //         id -> Int8,
    //         username -> Text,
    //         name -> Text,
    //         #[max_length = 100]
    //         email -> Varchar,
    //         created_at -> Timestamptz,
    //         updated_at -> Timestamptz,
    //     }

    // see this issue: https://github.com/diesel-rs/diesel/issues/952
    match ft_user::table
        .filter(ft_user::id.eq(user_data.id))
        .select(ft_user::id)
        .first::<i64>(conn)
    {
        Ok(v) => {
            tracing::info!("found user_id for username");
            return Ok(v);
        }
        Err(diesel::result::Error::NotFound) => {
            tracing::info!("not found user_id for username");
        }
        Err(e) => return Err(e.into()),
    };

    let user = ft_common::User {
        id: user_data.id,
        username: user_data.username.to_string(),
        name: user_data.name.to_string(),
        email: user_data.email.to_string(),
        created_at: *now,
        updated_at: *now,
    };

    tracing::info!(?user);

    diesel::insert_into(ft_user::table)
        .values(user)
        .returning(ft_user::id)
        .get_result::<i64>(conn)
        .map_err(|e| {
            tracing::error!("create_user insert failed: {e:?}");
            e.into()
        })
}

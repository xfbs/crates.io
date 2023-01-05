use crate::controllers::frontend_prelude::*;

use crate::models::{CrateOwner, OwnerKind, User};
use crate::schema::{crate_owners, crates, users};
use crate::sql::lower;
use crate::views::EncodablePublicUser;

/// Handles the `GET /users/:user_id` route.
pub fn show(req: ConduitRequest) -> AppResult<Json<Value>> {
    use self::users::dsl::{gh_login, id, users};

    let name = lower(req.param("user_id").unwrap());
    let conn = req.app().db_read_prefer_primary()?;
    let user: User = users
        .filter(lower(gh_login).eq(name))
        .order(id.desc())
        .first(&*conn)?;

    Ok(Json(json!({ "user": EncodablePublicUser::from(user) })))
}

/// Handles the `GET /users/:user_id/stats` route.
pub fn stats(req: ConduitRequest) -> AppResult<Json<Value>> {
    use diesel::dsl::sum;

    let user_id = req
        .param("user_id")
        .unwrap()
        .parse::<i32>()
        .map_err(|err| err.chain(bad_request("invalid user_id")))?;
    let conn = req.app().db_read_prefer_primary()?;

    let data: i64 = CrateOwner::by_owner_kind(OwnerKind::User)
        .inner_join(crates::table)
        .filter(crate_owners::owner_id.eq(user_id))
        .select(sum(crates::downloads))
        .first::<Option<i64>>(&*conn)?
        .unwrap_or(0);

    Ok(Json(json!({ "total_downloads": data })))
}

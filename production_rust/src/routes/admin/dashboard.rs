use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn admin_dashboard(
    session: TypedSession,
    connection_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &connection_pool)
            .await
            .map_err(e500)?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Admin dashboard</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 0;
            background-color: #3B5323;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
        }}

        .dashboard {{
            background-color: #ffffff;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
        }}

        h1 {{
            text-align: center;
            color: #3B5323;
        }}

        o1 {{
            list-style-type: none;
            padding-left: 0;
        }}

        li {{
            margin-bottom: 10px;
        }}
        
        li a:hover {{
            text-decoration: underline;
        }}

        .form-container {{
            background-color: #F8F8F8;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
            max-width: 400px;
            margin: 0 auto;
        }}


        form {{
            display: flex;
            flex-direction: column;
        }}

        label {{
            margin-bottom: 10px;
            color: #3B5323;
        }}

        input[type="submit"] {{
            padding: 10px 20px;
            background-color: #3B5323;
            color: #ffffff;
            border: none;
            border-radius: 3px;
            cursor: pointer;
        }}
    </style>
</head>
<body>
    <div class="dashboard">
        <h1>Welcome {username}!</h1>
        <p>Available actions:</p>
        <ol>
            <li><a href="/admin/newsletter">Send a newsletter</a></li>
            <li><a href="/admin/password">Change password</a></li>
            <li><a href="/admin/settings">Manage keys</a></li>
            <li><a href="/home">Go to home</a></li> 
        </ol>
        <form name ="logoutForm" action="/admin/logout" method="post">
            <input type="submit" value="Logout">
        </form>
    </div>
</body>
</html>"#
        )))
}

#[tracing::instrument(name = "Get username", skip(connection_pool))]
pub async fn get_username(
    user_id: Uuid,
    connection_pool: &PgPool,
) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username FROM users WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(connection_pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}

use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-i">
    <title>Change Password</title>
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

        .form-container {{
            background-color: #F8F8F8;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
            max-width: 400px;
            margin: 0 auto;
        }}

        h1 {{
            text-align: center;
            color: #3B5323;
        }}

        form {{
            display: flex;
            flex-direction: column;
        }}

        label {{
            margin-bottom: 10px;
            color: #3B5323;
        }}

        input[type="text"], textarea {{
            height: 20px;
            width: 100%;
            padding: 5px;
            border: 1px solid #ccc;
            border-radius: 3px;
            margin-bottom: 5px;
        }}

        button[type="submit"], button[type="button"] {{
            padding: 10px 20px;
            background-color: #3B5323;
            color: #ffffff;
            border: none;
            border-radius: 3px;
            curson: pointer;
        }}

        button:hover {{
            background-color: #2A3F1B;
        }}

        p a {{
            color: #3B5323;
            text-decoration: none;
        }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>Change Admin Password</h1>
        {msg_html}
        <form action="/admin/password" method="post">
            <label>Current password:
                <input
                    type="pasword"
                    placeholder="Enter current password"
                    name="current_password"
                >
            </label>
            <br>
            <label>New password:
                <input
                    type="password"
                    placeholder="Enter new password"
                    name="new_password"
                >
            </label>
            <br>
            <div class="button-container">    
                <button type="submit">Change password</button>
                <a href="/admin/dashboard"><button type="button">Back</button>
            </div>
        </form>
</body>
</html>"#,
        )))
}

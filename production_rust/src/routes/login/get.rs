use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Login</title>
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
        </style>
    </head>
    <body>
        <div class="form-container">
            <h1>Admin Login</h1>
            {error_html}
            <form action="/login" method="post">
                <label>Username:<br>
                    <input
                        type="text"
                        placeholder="Enter Username"
                        name = "username"
                    >
                </label>
                <label>Password:<br>
                    <input
                        type="password"
                        placeholder="Enter Password"
                        name="password"
                    >
                </label>
                
                <button type="submit">Login</button>
            </form>
    </body>
</html>"#,
        ))
}

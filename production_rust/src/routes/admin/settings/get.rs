use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn manage_settings_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
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
    <title>Manage API</title>
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

        .input-group {{
            margin-bottom: 20px;
        }}

        .input-group label {{
            display: block;
        }}

        .input-group input[type="password"] {{
            width: calc(100% - 12px);
            padding: 5px 6px;
            border: 1px solid #ccc;
            border-radius: 3px;
        }}

        .input-group button {{
            background-color: #F3F3F3;
            border: none;
            color: #000;
            padding: 0;
        }}

        .button-container {{
            display: flex;
            justify-content: center;
        }}

        button[type="submit"], button[type="button"] {{
            margin-right: 10px;
            padding: 10px 20px;
            background-color: #3B5323;
            color: #ffffff;
            border: none;
            border-radius: 3px;
            curson: pointer;
        }}

        button[type="submit"]:hover, button[type="button"]:hover {{
            background-color: #2A3F1B;
        }}

        p a {{
            color: #3B5323;
            text-decoration: none;
        }}
    </style>
    <script>
        function togglePasswordVisibility(elementId) {{
            const passwordInput = document.getElementById(elementId);
            const toggleButton = document.getElementById(elementId + '-toggle');

            if (passwordInput.type == 'password') {{
                passwordInput.type = 'text';
                toggleButton.textContent = 'Hide';
            }} else {{
                passwordInput.type = 'password';
                toggleButton.textContent = 'Show';
            }}
        }}
    </script>
</head>
<body>
    <div class="form-container">
        <h1>Manage Keys</h1>
        {msg_html}
        <form action="/admin/settings" method="post">
            <div class="input-group">
                <label for="idempotency_key">Key:</label>
                <input type="text" 
                    id="idempotency_key"
                    name="idempotency_key"
                    placeholder="Enter an idempotency key"
                >
            </div>
            <div class="button-container">   
                <button type="submit">Restore</button>
                <button type="submit">Revoke</button>
                <input type="hidden" id="validity_input" name="validity">
                <a href="/admin/dashboard"><button type="button">Back</button>
            </div>
        </form>
</body>
</html>"#,
        )))
}

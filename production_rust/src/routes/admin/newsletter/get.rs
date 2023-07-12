use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn new_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-i">
    <title>Submit a Newsletter</title>
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

        .input-group input[type="text"], 
        .input-group textarea {{
            height: 100px;
            width: 100%;
            padding: 5px;
            border: 1px solid #ccc;
            border-radius: 3px;
            margin-bottom: 5px;
        }}

        .input-group .title-textarea {{
            height: 20px;
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
        <h1>Submit a Newsletter</h1>
        {msg_html}
        <form action="/admin/newsletter" method="post">
            <div class="input-group">
                <label for="title"> Title:<br></label> 
                <textarea id="title" 
                    name="title" 
                    placeholder="Enter the Newsletter title" 
                    class="title-textarea">
                </textarea>
            </div>
            <br>
            <div class="input-group">
                <label for="text_content"> Text Submission:</label> 
                <textarea id="text_content" 
                    name="text_content" 
                    placeholder="Enter the content in plain text" 
                    class="input-group">
                </textarea>
            </div>
            <br>
            <div class="input-group">
                <label for="html_content"> HTML Submission:</label> 
                <textarea id="html_content" 
                    name="html_content" 
                    placeholder="Enter the content in HTML format" 
                    class="input-group">
                </textarea>
            </div>
            <br>
            <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
            <div class="button-container">
                <button type="submit">Publish</button>
                <a href="/admin/dashboard"><button type="button">Back</button>
            </div>
        </form>
    </div>
</body>
</html>"#,
        )))
}

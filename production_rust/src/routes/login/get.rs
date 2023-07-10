use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    // add a "back" nav to return to /home
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
                    /* background-color: #E4F9F5; */
                    margin: 0;
                    text-align: center;
                    font-family: "Merriweather", serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                }}
                
                hr {{
                    border: dotted #EAF6F6 6px;
                    border-bottom: none;
                    width: 50%;
                    margin: 100px auto;
                }}
                
                h1, h2 {{
                    color: #66BFBF;
                    font-size: 2.5rem;
                    font-family: "Montserrat", sans-serif;
                    font-weight: normal;
                }}
                
                h3 {{
                    color: #11999E;
                    font-family: "Montserrat", sans-serif;
                }}
            
                .center-container {{
                    display: flex;
                    width: 100%;
                    margin-bottom: 20px;
                    padding: 20px;
                    background: #f2f2f2;
                    border-radius: 5px;
                }}

                .form-container {{
                    background-color: #f2f2f2;
                    padding: 20px;
                    border-radius: 5px;
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
        <div class="center-container">
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
            </div>
        </div>
    </body>
</html>"#,
        ))
}

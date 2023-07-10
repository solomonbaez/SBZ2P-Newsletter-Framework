use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn get_subscribe(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale = 1.0">
    <title>Subscribe to the Blog</title>
    <style>
        body {{
            /* background-color: #E4F9F5; */
            margin: 0;
            text-align: center;
            font-family: "Merriweather", serif;
        }}
        
        hr {{
            border: dotted #EAF6F6 6px;
            border-bottom: none;
            width: 50%;
            margin: 100px auto;
        }}
        
        h1 {{
            color: #66BFBF;
            font-size: 5.625rem;
            margin: 50px auto 0 auto;
            font-family: "Sacramento", cursive;
        }}
    
        h2 {{
            color: #66BFBF;
            font-size: 2.5rem;
            font-family: "Montserrat", sans-serif;
            font-weight: normal;
        }}
        
        h3 {{
            color: #11999E;
            font-family: "Montserrat", sans-serif;
        }}
       
        nav u1 {{
            list-style: none;
            display: flex;
            justify-content: center;
            margin-top: 20px;
        }}

        nav u1 li {{
            margin-right: 20px;
        }}
        
        nav u1 li a {{
            color: #777;
            text-decoration: none;
            transition: color 0.3s ease;
        }}
        
        nav u1 li a:hover {{
            color: #333;
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
            box-sizing: border-box
            display: flex;
            justify-content: center;
            height: 20px;
            width: 200px;
            padding: 5px;
            border: 1px solid #ccc;
            border-radius: 3px;
            margin-bottom: 5px;
        }}

        button[type="submit"], button[type="button"] {{
            box-sizing: border-box
            display: flex;
            justify-content: center;
            padding: 10px 20px; 
            border: 1px;
            border-radius: 3px;
            width: 200px;
            cursor: pointer;
            background-color: #007bff;
            color: #fff;
            border-radius: 3px;
            transition: background-color 0.3s ease;
        }}

        button:hover {{
            background-color: #2A3F1B;
        }}
    </style>
</head>
<body>
    <header>
        <nav>
            <u1>
                <li><a href="/home">Home</a></li>
                <li><a href="/blog">Blog</a></li>
                <li><a href="/subscribe">Subscribe</a></li>
                <li><a href="/contact">Contact</a></li>
            </u1>
        </nav>
    </header>
        
    <main>
        <section>
            <h2>Subscribe</h2>
            {msg_html}
        </section>
        <section>
            <div class="center-container">
                <div class="form-container">
                    {msg_html}
                    <form action="/subcriptions" method="post">
                        <label>Email:<br>
                            <input
                                type="text"
                                placeholder="enter your email"
                                name = "email"
                            >
                        </label>
                        <button type="submit">Subscribe</button>
                    </form>
                </div>
            </div>
        </section>
    </main>

    <footer>
        <p>&copy; 2023 Solomon Baez</p>
        <p><a href="/login">admin login</a></p>
    </footer>
</body>
</html>"#
        )))
}

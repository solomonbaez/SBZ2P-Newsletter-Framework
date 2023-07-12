use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn blog(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
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
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Blog</title>
                <style>
                    body {{
                        margin: 0;
                        text-align: center;
                        font-family: "Merriweather", serif;
                        background-color: #111;
                        color: #fff;
                    }}
                    
                    hr {{
                        border: dotted #444 6px;
                        border-bottom: none;
                        width: 50%;
                        margin: 100px auto;
                    }}
                    
                    h1 {{
                        color: #007bff;
                        font-size: 5.625rem;
                        margin: 50px auto 0 auto;
                        font-family: "Sacramento", cursive;
                    }}
                    
                    h2 {{
                        color: #007bff;
                        font-size: 2.5rem;
                        font-family: "Montserrat", sans-serif;
                        font-weight: normal;
                    }}
                    
                    h3 {{
                        color: #11999E;
                        font-family: "Montserrat", sans-serif;
                    }}
                    
                    p {{
                        font-family: "Roboto", sans-serif;
                        font-size: 16px;
                        color: #ccc;
                    }}
                    
                    nav ul {{
                        list-style: none;
                        display: flex;
                        justify-content: center;
                        margin-top: 20px;
                    }}
                    
                    nav ul li {{
                        margin-right: 20px;
                    }}
                    
                    nav ul li a {{
                        color: #fff;
                        text-decoration: none;
                        transition: color 0.3s ease;
                    }}
                    
                    nav ul li a:hover {{
                        color: #007bff;
                    }}
                    
                    .blog-post {{
                        margin-bottom: 20px;
                        padding: 20px;
                        background: #222;
                        border-radius: 5px;
                        display: inline-block;
                        text-align: left;
                    }}
                    
                    .blog-post h3 {{
                        font-size: 24px;
                        margin-bottom: 10px;
                        color: #fff;
                    }}
                    
                    .blog-post p {{
                        margin-bottom: 10px;
                    }}
                    
                    button[type="button"] {{
                        display: inline-block;
                        padding: 8px 12px;
                        justify-content: center;
                        padding: 10px 20px;
                        border: 1px;
                        border-radius: 3px;
                        width: 125px;
                        cursor: pointer;
                        text-decoration: none;
                        background-color: #007bff;
                        color: #fff;
                        border-radius: 3px;
                        transition: background-color 0.3s ease;
                    }}
                    
                    button:hover {{
                        background-color: #003d5a;
                    }}
                </style>
            </head>
            <body>
                <header>
                    <nav>
                        <ul>
                            <li><a href="/home">Home</a></li>
                            <li><a href="/blog">Blog</a></li>
                            <li><a href="/subscriptions">Subscribe</a></li>
                            <li><a href="/contact">Contact</a></li>
                        </ul>
                    </nav>
                </header>
                    
                <main>
                    <section>
                        <h2>Welcome to my Blog!</h2>
                        {msg_html}
                    </section>
                    <section>
                        <div class="blog-post">
                            <h3>Blog Post Title</h3>
                            <p>Blog content summary.</p>
                            <a href="/blog/post-slub"><button type="button">Read More</button></a>
                        </div>
                        <div class="blog-post">
                            <h3>Blog Post Title</h3>
                            <p>Blog content summary.</p>
                            <a href="/blog/post-slub"><button type="button">Read More</button></a>
                        </div>
                    </section>
                </main>
            
                <footer>
                    <p>&copy; 2023 Solomon Baez</p>
                    <p><a href="/login">Admin Login</a></p>
                </footer>
            </body>
            </html>
            "#
        )))
}

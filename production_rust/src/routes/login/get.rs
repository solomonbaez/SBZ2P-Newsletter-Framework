use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    // add a "back" nav to return to /home
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Subscribe to the Blog</title>
                <style>
                    body {{
                        margin: 0;
                        text-align: center;
                        font-family: "Merriweather", serif;
                        background-color: #111;
                        color: #fff;
                    }}

                    section {{
                        display: flex;
                        justify-content: center;
                        align-items: center;
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
            
                    .link-container {{
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        margin-bottom: 20px;
                        padding: 20px 100px;
                        background: #222;
                        border-radius: 5px;
                        text-align: center;
                        max-width: 800px;
                        margin: 0 auto;
                        overflow-wrap: break-word;
                        width: calc(100% - 40px);
                        margin-left: auto;
                        margin-right: auto;
                    }}
    
                    .link-container::before {{
                        content: "";
                        display: inline-block;
                        width: calc(50vw - 350px);
                        min-width: 20px;
                    }}
    
                    .link-container::after {{
                        content: "";
                        display: inline-block;
                        width: calc(50vw - 350px);
                        min-width: 20px;
                    }}
        
                    .link-container h3 {{
                        font-size: 2.25rem;
                        margin-bottom: 10px;
                        color: #fff;
                    }}
        
                    .link-container p {{
                        margin-bottom: 10px;
                    }}
            
                    label {{
                        margin-bottom: 10px;
                        color: #fff;
                    }}

                    .form-container {{
                        display: grid;
                        grid-template-columns: 1fr 1fr;
                        gap: 20px;
                        padding: 20px;
                        border-radius: 5px;
                        max-width: 400px;
                        margin-top: 20px;
                        margin-bottom: 50px;
                    }}

                    .input-column {{
                        display: flex;
                        flex-direction: column;
                    }}

                    .input-column label {{
                        margin-bottom: 5px
                    }}

                    .input-column input {{
                        width: 100%
                    }}

                    input[type="text"], 
                    textarea {{
                        box-sizing: border-box;
                        display: flex;
                        justify-content: center;
                        height: 50px;
                        width: 200px;
                        padding: 5px;
                        border: 1px solid #ccc;
                        border-radius: 3px;
                        margin-bottom: 5px;
                    }}

                    input[type="password"], 
                    textarea {{
                        box-sizing: border-box;
                        display: flex;
                        justify-content: center;
                        height: 50px;
                        width: 200px;
                        padding: 5px;
                        border: 1px solid #ccc;
                        border-radius: 3px;
                        margin-bottom: 5px;
                    }}
            
                    button[type="submit"] {{
                        grid-column: 1 / span 2;
                        box-sizing: border-box;
                        display: flex;
                        justify-content: center;
                        padding: 10px 20px;
                        border: 1px;
                        border-radius: 3px;
                        width: 420px;
                        cursor: pointer;
                        background-color: #007bff;
                        color: #fff;
                        border-radius: 3px;
                        transition: background-color 0.3s ease;
                    }}
            
                    button:hover {{
                        background-color: #003d5a;
                    }}
                    
                    h1::after {{
                        content: "_";
                        display: inline-block;
                        width: 6px;
                        height: 40px;
                        background-color: #222;
                        animation: blink-animation 1.5s infinite;
                    }}

                    h2::after {{
                        content: "_";
                        display: inline-block;
                        width:6px;
                        height: 40px;
                        background-color: #111;
                        animation: blink-animation 1.5s infinite;
                    }}
                    
                    @keyframes blink-animation {{
                        0% {{ opacity: 1; }}
                        50% {{ opacity: 0; }}
                        100% {{ opacity: 1; }}
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
                        <h2>Admin</h2>
                    </section>
                    <section>
                        {msg_html}
                    </section>
                    <section>
                        <div class="link-container">
                                <form action="/login" method="post" class="form-container">
                                    <div class="input-column">
                                        <label for"username"><h3>Username:</h3><br>
                                        <input id="username"
                                            name="username"
                                            type="text"
                                            placeholder="admin username"
                                        >
                                        </label>
                                    </div>
                                    <div class="input-column">
                                        <label for="password"><h3>Password:</h3><br>
                                            <input id="password"
                                                name="password"
                                                type="password"
                                                placeholder="admin password"
                                            >
                                        </label>
                                    </div>
                                    <button type="submit">Login</button>
                                </form>
                                </div>
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
        ))
}

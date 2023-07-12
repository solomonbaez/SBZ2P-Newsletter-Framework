use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn home(flash_messages: IncomingFlashMessages) -> Result<HttpResponse, actix_web::Error> {
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
                <title>Solomon Baez</title>
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
                        font-family: "Montserrat", sans-serif;
                        font-weight: bold;
                        display: inline-block;
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

                    h3 {{
                        color: #007bff;
                        font-family: "Montserrat", sans-serif;
                    }}

                    p {{
                        font-family: "Roboto", sans-serif;
                        font-size: 16px;
                        color: #ccc;
                        line-height: 1.6;
                    }}
                    
                    .skill-row {{
                        width: 50%;
                        margin: 100px auto 100px auto;
                        text-align: left;
                        line-height: 2;
                    }}

                    .experience-row {{
                        display: flex
                        align-items: center;
                        width: 50%;
                        margin: 100px auto 100px auto;
                        text-align: left;
                        line-height: 2;
                    }}

                    .experience-row p {{
                        font-size: 32 px;
                    }}

                    .top-container {{
                        background-color: #222;
                        position: relative;
                        padding-top: 25px;
                        padding-bottom: 55px;
                    }}

                    .bottom-container {{
                        background-color: #111;
                        padding: 50px 0px 20px;
                    }}

                    .profile-container {{
                        text-align: center;
                        width: 50%;
                        margin: 0 auto;
                    }}

                    .work {{
                        color: grey;
                        text-decoration: underline;
                    }}

                    .top-cloud {{
                        position: absolute;
                        right: 300px;
                        top: 50px;
                    }}

                    .sub-cloud {{
                        position: absolute;
                        left: 300px;
                        bottom: 300px;
                    }}

                    .python-img {{
                        width: 50%;
                        float: right;
                        margin-right: 30px;
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
            <br>
            <div class="top-container">
                <h1>I'm Solomon</h1>
                <h3>Co-Founder and former CEO of <strong>PediaNourish LLC</strong></h3>
                <h3><span class="work">Computer Scientist,</span> <span class="work">Bioengineer</span></h3>
            </div>

            <div class="middle-container">
                {msg_html}
                <div class="profile-container">
                    <h2>Hello</h2>
                    <p>I am a bioengineer and software developer with four years of research
                        experience in medical device software and product development 
                        at Oregon State University.
                    </p>
                </div>
                <hr>
                <div class="skills">
                    <h2>My Experience</h2>
                    <div class="experience-row">
                        <p><strong style="color: #007bff;">2019-2023:</strong> PediaNourish LLC</p>
                        <p><strong style="color: #007bff;">2018-2023:</strong> Higgins/Dallas Lab, OSU</p>
                        <p><strong style="color: #007bff;">2018-2018:</strong> Higgins Lab, OSU</p>
                    </div>
                    <hr>
                    <h2>My Skills</h2>
                    <div class="skill-row">
                        <img class="python-img" src="https://i.giphy.com/media/E4kjYvAnTjh45ML3TO/giphy.webp" alt="python-img">
                        <h3><strong>Programming</strong>: </h3>
                        <p>Python; Rust; PostgresSQL; MySQL</p>
                        <br>
                        <h3><strong>Skills</strong>: </h3>
                        <p>Data Science/Engineering</p>
                        <p>Engineering Product Design</p>
                        <p>Engineering Product Commercialization</p>
                    </div>
                </div>
            </div>

            <footer>
                <p>&copy; 2023 Solomon Baez</p>
                <p><a href="/login">admin login</a></p>
            </footer>

            </body>
            </html>"#
        )))
}

pub async fn contact_me(
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
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Contact Me</title>
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
                    
                    .link-container {{
                        margin-bottom: 20px;
                        padding: 20px;
                        background: #222;
                        border-radius: 5px;
                    }}
                    
                    .link-container h3 {{
                        font-size: 24px;
                        margin-bottom: 10px;
                        color: #fff;
                    }}
                    
                    .link-container p {{
                        margin-bottom: 10px;
                    }}
                    
                    button[type="button"] {{
                        display: inline-block;
                        padding: 8px 12px;
                        justify-content: center;
                        padding: 10px 20px;
                        border: 1px;
                        border-radius: 3px;
                        width: 150px;
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
                        <h2>Contact Me</h2>
                        {msg_html}
                    </section>
                    <section>
                        <div class="link-container">
                            <h3>Work Links</h3>
                            <br>
                            <p><a href="https://github.com/solomonbaez"><button type="button">My GitHub</button></a></p>
                            <br>
                            <p><a href="https://www.linkedin.com/in/solomonbaez"><button type="button">My Linkedin</button></a></p>
                            <br>
                            <p><a href="https://fishbowlapp.com/fb/solomon-baez"><button type="button">My Fishbowl</button></a></p>
                            <br>
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

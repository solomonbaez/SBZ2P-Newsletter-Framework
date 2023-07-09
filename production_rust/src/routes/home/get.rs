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
        .body(format!(r#"
        <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset ="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale = 1.0">
    <title>Solomon Baez</title>
    <link rel ="stylesheet" type ="text/css" href ="./home/stylesheet.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Merriweather&family=Montserrat&family=Sacramento&display=swap" rel="stylesheet">
    <style>
        body {{
            /* background-color: #E4F9F5; */
            margin: 0;
            text-align: center;
            font-family: "Merriweather", serif;
        }}
        
        .skill-row {{
            width: 50%;
            margin: 100px auto 100px auto;
            text-align: left;
            line-height: 2;
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
        
        .top-container {{
            background-color: #E4F9F5;
            position: relative;
            padding-top: 25px;
            padding-bottom: 55px;
        }}
        
        .bottom-container {{
            background-color: #66BFBF
            padding: 50px 0px 20px;
        }}
        
        #pfp {{
            padding-top: 25px;
            border-radius: 100%;
        }}
        
        .bio {{
            text-align: center;
            width: 50%;
        }}
        
        /* inline, vs block display */
        .work {{
            color: grey;
            text-decoration: underline;
        }}
        
        .top-cloud{{
            position: absolute;
            right: 300px;
            top: 50px;
        }}
        
        .sub-cloud{{
            position: absolute;
            left: 300px;
            bottom: 300px;
        }}
        
        .computer-img {{
            width: 30%;
            float: left;
            margin-right: 30px;
        }}
        .python-img {{
            width: 50%;
            float: right;
            margin-right: 30px;
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
<br>
<!-- content division element-->
<div class="top-container">
    <h1>I'm Solomon</h1>
    <h3>Co-Founder and former CEO of <strong>PediaNourish LLC</strong></h3>
    <h3><span class="work">Computer Scientist,</span>
        <span class="work">Bioengineer</span></h3>
</div>
</body>
<div class="middle-container">
    {msg_html}
    <div class="profile">
        <h2>Hello.</h2>
        <p>I am a bioengineer and software developer with four years of research
            experience in medical device development at Oregon State University.
        </p>
        <p>I have recently taken a hiatus from PediaNourish LLC to gain more professional experience.</p>
    </div>
    <hr>
    <div class="skills">
            <h2>My Experience.</h2>
        <div class="skill-row">
            <img class="computer-img" src="https://i.giphy.com/media/iIGT8Y1rOYhBpdHh1C/giphy.webp" alt="code-img">
            <p>2018-2018: Higgins Lab, OSU</p>
            <p>2018-2023: Higgins and Dallas Joint Lab, OSU</p>
            <p>2019-2023: PediaNourish LLC</p>
        </div>
        <hr>
        <h2>My Skills.</h2>
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
    <meta name="viewport" content="width=device-width, initial-scale = 1.0">
    <title>Contact Me</title>
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
        
        nva u1 {{
            margin-top: 20px
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
        .link-container {{
            margin-bottom: 20px;
            padding: 20px;
            background: #f2f2f2;
            border-radius: 5px;
        }}
        
        .link-container h3 {{
            font-size: 24px;
            margin-bottom: 10px;
        }}
        
        .link-container {{
            margin-bottom: 10px;
        }}
        
        .link-container a {{
            display: inline-block;
            padding: 8px 12px;
            background-color: #007bff;
            color: #fff;
            text-decoration: none;
            border-radius: 3px;
            transition: background-color 0.3s ease;
            width: 125px;
            text-align: center;
        }}
        
        .link-container a:hove {{
            background-color: #0056b3;
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
            <h2>Contact Me</h2>
            {msg_html}
        </section>
        <section>
            <div class = "link-container">
                <h3>Work Links</h3>
                <br>
                <p><a href="https://github.com/solomonbaez">My GitHub</a><p>
                <br>
                <p><a href="https://www.linkedin.com/in/solomonbaez">My Linkedin</a></p>
                <br>
                <p><a href="https://fishbowlapp.com/fb/solomon-baez">My Fishbowl</a></p> 
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

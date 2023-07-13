use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use actix_web_lab::extract::Path;
use sqlx::PgPool;
use std::fmt::Write;
// use chrono::NaiveDateTime;
use uuid::Uuid;

pub async fn blog(
    request: HttpRequest,
    connection_pool: web::Data<PgPool>,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let newsletter_posts = get_recent_newsletters(connection_pool)
        .await
        .map_err(e500)?;

    let blog_posts_html: Vec<String> = newsletter_posts
        .iter()
        .map(|blog_post| {
            let mut words = blog_post.text_content.split_whitespace();
            let mut display_text = words.by_ref().take(10).collect::<Vec<_>>().join(" ");
            if words.next().is_some() {
                display_text.push_str("..."); // Add ellipsis if there are more words
            }
            format!(
                r#"
                <div class="blog-post">
                    <h3>{}</h3>
                    <p>{}...</p>
                    <a href="{}">
                        <button type="button">Read More</button>
                    </a>
                </div>
                "#,
                blog_post.title,
                display_text,
                request
                    .url_for("blog_post", [&blog_post.id.to_string()])
                    .expect("Failed to generate blog post URL."),
            )
        })
        .collect();

    let blog_html = format!(
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

                    // section {{
                    //     display: flex;
                    //     justify-content: center;
                    //     align-items: center;
                    // }}
                    
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

                    .blog-posts {{
                        display: grid;
                        grid-template-columns: repeat(2, 1fr); /* Two columns */
                        grid-gap: 10px; /* Gap between columns */
                    }}
    
                    .blog-post {{
                        max-width: 400px;
                        width: calc(100% - 40px);
                        margin-left: auto;
                        margin-right: auto;
                        height: 200px; /* Adjust the height as per your preference */
                        margin-bottom: 20px;
                        padding: 20px;
                        background: #222;
                        border-radius: 5px;
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
                        <h2>Welcome</h2>
                    </section>
                    <section>
                        {msg_html}
                    </section>
                    <section>
                        {blog_posts_html}
                    </section>
                </main>
            
                <footer>
                    <p>&copy; 2023 Solomon Baez</p>
                    <p><a href="/login">Admin Login</a></p>
                </footer>
            </body>
            </html>
            "#,
        msg_html = msg_html,
        blog_posts_html = blog_posts_html.join("\n"),
    );

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(blog_html))
}

#[allow(dead_code)]
pub struct NewsletterPost {
    pub id: Uuid,
    pub title: String,
    pub text_content: String,
    pub html_content: String,
    pub published_at: String,
}

pub async fn blog_post_handler(
    Path(newsletter_id): Path<Uuid>,
    connection_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let newsletter_post = get_newsletter_post_by_id(connection_pool, newsletter_id)
        .await
        .map_err(e500)?;

    let post_title = newsletter_post.title.to_string();
    let post_content = newsletter_post.html_content;

    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{post_title}</title>
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
                    margin-bottom: 20px;
                    padding: 20px 100px;
                    background: #222;
                    border-radius: 5px;
                    text-align: left;
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
                    <h2>{post_title}</h2>
                </section>
                <section>
                    <div class=link-container>
                        {post_content}
                    </div>
                </section>
            </main>
        
            <footer>
                <p>&copy; 2023 Solomon Baez</p>
                <p><a href="/login">Admin Login</a></p>
            </footer>
        </body>
        </html>
        "#,
    );

    Ok(HttpResponse::Ok().body(html_content))
}

pub async fn get_recent_newsletters(
    connection_pool: web::Data<PgPool>,
) -> Result<Vec<NewsletterPost>, sqlx::Error> {
    const RECENT_POSTS: i64 = 2;

    let query = sqlx::query!(
        r#"
        SELECT newsletter_issue_id, title, text_content, html_content, published_at
        FROM newsletter_issues
        ORDER BY published_at DESC
        LIMIT $1
        "#,
        RECENT_POSTS,
    );

    let rows = query.fetch_all(&**connection_pool).await?;

    let newsletter_posts = rows
        .into_iter()
        .map(|row| NewsletterPost {
            id: row.newsletter_issue_id,
            title: row.title,
            text_content: row.text_content,
            html_content: row.html_content,
            published_at: row.published_at,
        })
        .collect();

    Ok(newsletter_posts)
}

pub async fn get_newsletter_post_by_id(
    connection_pool: web::Data<PgPool>,
    newsletter_id: Uuid,
) -> Result<NewsletterPost, sqlx::Error> {
    let query = sqlx::query!(
        r#"
        SELECT newsletter_issue_id, title, text_content, html_content, published_at
        FROM newsletter_issues
        WHERE newsletter_issue_id = $1
        "#,
        newsletter_id
    );

    let post = query.fetch_one(&**connection_pool).await?;
    let newsletter_post = NewsletterPost {
        id: post.newsletter_issue_id,
        title: post.title,
        text_content: post.text_content,
        html_content: post.html_content,
        published_at: post.published_at,
    };

    Ok(newsletter_post)
}

use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::PgPool;
use std::fmt::Write;
// use chrono::NaiveDateTime;
use uuid::Uuid;

pub async fn blog(
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

    let blog_posts_html = newsletter_posts
        .iter()
        .map(|blog_post| {
            format!(
                r#"
                <div class="blog-post">
                    <h3>{}</h3>
                    <p>{}...</p>
                    <a href="/blog/post-{}"><button type="button">Read More</button></a>
                </div>
                "#,
                blog_post.title, blog_post.text_content, blog_post.html_content
            )
        })
        .collect::<String>();

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
        blog_posts_html = blog_posts_html
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

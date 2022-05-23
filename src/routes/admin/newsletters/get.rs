use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn send_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let msg_html: String = flash_messages
        .iter()
        .map(|m| format!("<p><i>{}</i></p>", m.content()))
        .collect();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Change Password</title>
    </head>
    <body>
        {msg_html}
        <form action="/admin/newsletters" method="post">
            <h1>Send a new newsletter</h1>
            <label>Title
                <input
                    type="text"
                    placeholder="Add title"
                    name="title"
                >
            </label>
            <br>
            <h2>Content</h2>
            <label>Html content</label><br>
                <textarea
                    placeholder="Add html newsletter"
                    name="html"
                ></textarea>
            <br>
            <label>Plain text content</label><br>
            <textarea
                placeholder="Add plain text content"
                name="text"
            ></textarea>
            <br>
            <button type="submit">Send newsletter</button>
        </form>
        <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>"#,
        )))
}

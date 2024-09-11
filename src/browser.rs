use chromiumoxide::{Browser, BrowserConfig};
use futures::stream::StreamExt;
use tokio::task;

use crate::actions::login::login;

pub async fn start_browser() -> Result<(), Box<dyn std::error::Error>> {
    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let mut page = browser.new_page("https://forum.mafiascum.net").await?;
    login(&mut page).await?;

    page.wait_for_navigation().await?;

    page.goto("https://forum.mafiascum.net/ucp.php?i=ucp_profile&mode=signature")
        .await?;

    page.wait_for_navigation().await?;

    let signature_field = page.find_element("textarea#signature").await?;
    let signature_value: String = page
        .evaluate("document.querySelector('textarea#signature').value")
        .await?
        .into_value()?;

    signature_field.focus().await?;

    page.evaluate(r#"document.querySelector('textarea#signature').value = 'test'"#)
        .await?;

    page.evaluate(
        r#"
        const textarea = document.querySelector('textarea#signature');
        const event = new KeyboardEvent('keydown', {
            key: 'Enter',
            code: 'Enter',
            keyCode: 13,
            which: 13,
            ctrlKey: true,
            bubbles: true
        });
        textarea.dispatchEvent(event);
    "#,
    )
    .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    page.wait_for_navigation().await?;

    browser.close().await?;
    handle.await?;

    Ok(())
}

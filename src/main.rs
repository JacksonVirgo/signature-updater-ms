use browser::start_browser;
use chrono::Local;
use tokio_cron_scheduler::{Job, JobScheduler};

mod actions;
mod browser;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    match start_scheduler().await {
        Ok(_) => {}
        Err(_) => {}
    }
}

async fn start_scheduler() -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;

    let job = Job::new_async("*/45 * * * * *", |_uuid, _l| {
        Box::pin(async {
            let now = Local::now();
            println!("Task executed at: {}", now.format("%Y-%m-%d %H:%M:%S"));
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    match start_browser().await {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}

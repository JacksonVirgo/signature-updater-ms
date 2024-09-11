use chrono::Local;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    match start_scheduler().await {
        Ok(_) => {}
        Err(_) => {}
    }
}

async fn start_scheduler() -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;

    let job = Job::new_async("*/15 * * * * *", |_uuid, _l| {
        Box::pin(async {
            let now = Local::now();
            println!("Task executed at: {}", now.format("%Y-%m-%d %H:%M:%S"));
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}

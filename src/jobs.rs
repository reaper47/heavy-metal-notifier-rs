use tracing::error;

pub async fn update_calendar() {
    if let Err(err) = crate::CALENDAR.lock().await.update().await {
        error!("Error updating calendar: {err}")
    }
}

use crate::{models, service::worktime};

pub struct Timer;

#[async_graphql::Object]
impl Timer {
    async fn timers(
        &self,
        ctx: &async_graphql::Context<'_>,
        employee_id: i32,
    ) -> async_graphql::Result<Vec<models::Worktime>> {
        let pool = ctx.data::<sqlx::Pool<sqlx::Postgres>>()?;

        worktime::get_timers(employee_id, pool)
            .await
            .map_err(|e| async_graphql::Error::new_with_source(e))
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer
    }
}

pub struct TimerMutation;

#[async_graphql::Object]
impl TimerMutation {
    async fn start_timer(
        &self,
        ctx: &async_graphql::Context<'_>,
        employee_id: i32,
        task_id: i32,
        worktype: models::WorktimeType,
    ) -> async_graphql::Result<models::Worktime> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        worktime::start_timer(employee_id, task_id, worktype, pool)
            .await
            .map_err(|e| async_graphql::Error::new_with_source(e))
    }

    async fn stop_timer(
        &self,
        ctx: &async_graphql::Context<'_>,
        worktime_id: i32,
    ) -> async_graphql::Result<models::Worktime> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        worktime::stop_timer(worktime_id, pool)
            .await
            .map_err(|e| async_graphql::Error::new_with_source(e))
    }

    async fn update_timer(
        &self,
        ctx: &async_graphql::Context<'_>,
        worktime_id: i32,
        task_id: Option<i32>,
        start_time: Option<chrono::DateTime<chrono::FixedOffset>>,
        end_time: Option<chrono::DateTime<chrono::FixedOffset>>,
        worktype: Option<models::WorktimeType>,
    ) -> async_graphql::Result<models::Worktime> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        worktime::update_timer(worktime_id, task_id, start_time, end_time, worktype, pool)
            .await
            .map_err(|e| async_graphql::Error::new_with_source(e))
    }
}

impl Default for TimerMutation {
    fn default() -> Self {
        TimerMutation
    }
}

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = uuid::Uuid::new_v4();
    // We are using the same interpolation syntax of `println`/`print` here!
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!("request_id {} - Saving subscriber to database", request_id);
    // `Result` has two variants: `Ok` and `Err`.
    // The first for successes, the second for failures.
    // We use a `match` statement to choose what to do based
    // on the outcome.
    // We will talk more about `Result` going forward!
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            //{:?}, the std::fmt::Debug format, to capture the query error
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

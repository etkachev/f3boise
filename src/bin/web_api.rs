use dotenvy::dotenv;
use f3_api_rs::configuration::get_configuration;
use f3_api_rs::shared::common_errors::AppError;
use f3_api_rs::web_api_run::Application;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    let config = get_configuration().expect("Failed to read config");
    let app = Application::build(config).await?;
    let application_task = tokio::spawn(app.run_until_stopped());
    // TODO let worker_task

    tokio::select! {
        o = application_task => report_exit("API", o),
    };

    Ok(())

    // let web_app = init_web_app().await;
    // let address = format!("{}:{}", LOCAL_URL, config.application_port);
    // let listener = std::net::TcpListener::bind(&address).expect("Could not bind to port");
    // run(web_app, listener)?.await
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_macros::FromRef;

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/ticket", post(create_ticket).get(list_tickets))
        .with_state(mc)
}
// region:    --- REST Handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

// endregion: --- REST Handlers

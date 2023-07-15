use axum::extract::{Path, State};
use axum::routing::{get, post};
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
        .route(
            "/ticket/:id",
            get(get_ticket).put(update_ticket).delete(delete_ticket),
        )
        .with_state(mc)
}
// region:    --- REST Handlers
async fn create_ticket(
    State(mut mc): State<ModelController>,
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

async fn get_ticket(
    Path(id): Path<u64>,
    State(mc): State<ModelController>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - get_ticket", "HANDLER");

    let ticket = mc.get_ticket(id).await?;

    Ok(Json(ticket))
}

async fn delete_ticket(
    Path(id): Path<u64>,
    State(mc): State<ModelController>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}

async fn update_ticket(
    Path(id): Path<u64>,
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - update_ticket", "HANDLER");

    let ticket = mc.update_ticket(id, ticket_fc).await?;

    Ok(Json(ticket))
}

// endregion: --- REST Handlers

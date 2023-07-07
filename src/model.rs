//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64 + 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store
            .iter()
            .filter_map(|ticket| ticket.clone())
            .collect::<Vec<Ticket>>();

        Ok(tickets)
    }

    pub async fn get_ticket(&self, id: u64) -> Result<Ticket> {
        let store = self.tickets_store.lock().unwrap();

        let ticket = store
            .get(id as usize - 1)
            .ok_or(Error::TicketNotFound)?
            .clone()
            .ok_or(Error::TicketNotFound)?;

        Ok(ticket)
    }

    pub async fn update_ticket(&self, id: u64, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store
            .get_mut(id as usize - 1)
            .ok_or(Error::TicketNotFound { id })?
            .as_mut()
            .ok_or(Error::TicketNotFound { id })?;

        ticket.title = ticket.title.clone();

        Ok(ticket.clone())
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize - 1).and_then(|t| t.take());

        ticket.ok_or(Error::TicketNotFound { id })
    }
}

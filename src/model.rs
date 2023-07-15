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
    tickets_store: DefaultType,
}

pub type DefaultType = Arc<Mutex<Vec<Option<Ticket>>>>;

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(self::ModelController::default())
    }
}

impl Default for ModelController {
    fn default() -> Self {
        Self {
            tickets_store: Arc::default(),
        }
    }
}

// CRUD trait for tickets_store
trait TicketStore {
    fn create_ticket(&mut self, ticket: TicketForCreate) -> Result<Ticket>;
    fn list_tickets(&self) -> Result<Vec<Ticket>>;
    fn get_ticket(&self, id: u64) -> Result<Ticket>;
    fn delete_ticket(&mut self, id: u64) -> Result<Ticket>;
    fn update_ticket(&mut self, id: u64, ticket: Ticket) -> Result<Ticket>;
}

impl TicketStore for DefaultType {
    fn create_ticket(&mut self, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.lock().unwrap();

        let id = store.len() as u64 + 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    fn list_tickets(&self) -> Result<Vec<Ticket>> {
        todo!()
    }

    fn get_ticket(&self, id: u64) -> Result<Ticket> {
        todo!()
    }

    fn delete_ticket(&mut self, id: u64) -> Result<Ticket> {
        todo!()
    }

    fn update_ticket(&mut self, id: u64, ticket: Ticket) -> Result<Ticket> {
        todo!()
    }
}
impl ModelController {
    pub async fn create_ticket(&mut self, ticket: TicketForCreate) -> Result<Ticket> {
        self.tickets_store.create_ticket(ticket)
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
            .ok_or(Error::TicketNotFound { id })?;

        // return Ok(Ticket) or Err
        ticket.clone().ok_or(Error::TicketNotFound { id })
    }

    pub async fn update_ticket(&self, id: u64, ticket_fu: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store
            .get_mut(id as usize - 1)
            .ok_or(Error::TicketNotFound { id })?
            .as_mut()
            .ok_or(Error::TicketNotFound { id })?;

        ticket.title = ticket_fu.title.clone();

        Ok(ticket.clone())
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize - 1).and_then(|t| t.take());

        ticket.ok_or(Error::TicketNotFound { id })
    }
}

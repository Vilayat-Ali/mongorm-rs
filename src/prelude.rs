use mongodb::{error::Result, options::ClientOptions, Client, Database};
use std::collections::HashMap;

pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
}

pub struct Connection {
    pub client: Client,
    pub connection_state: ConnectionState,
}

impl Connection {
    fn new(client: Client, connection_state: Option<ConnectionState>) -> Self {
        Self {
            client,
            connection_state: connection_state.unwrap_or(ConnectionState::Connecting),
        }
    }

    fn get_db(&self) -> Option<Database> {
        self.client.default_database()
    }
}

pub struct Mongorm {
    connections: Vec<Connection>,
    models: HashMap<&'static str, String>,
    options: MongormOptions,
}

pub struct MongormOptions {
    pub enable_backup: bool,
}

impl Default for MongormOptions {
    fn default() -> Self {
        Self {
            enable_backup: false,
        }
    }
}

impl Mongorm {
    pub fn builder(options: Option<MongormOptions>) -> Self {
        Self {
            connections: vec![],
            models: HashMap::new(),
            options: options.unwrap_or_default(),
        }
    }

    pub async fn add_connection<'a, S>(&mut self, conn_str: S) -> Result<&mut Self>
    where
        S: Into<&'a String>,
    {
        let options = ClientOptions::parse(conn_str.into()).await?;
        let client = Client::with_options(options)?;

        self.connections.push(Connection {
            client,
            connection_state: ConnectionState::Connecting,
        });

        Ok(self)
    }

    pub async fn add_model<'a, S, T>(&mut self, model_name: S, model: T) -> Result<&mut Self>
    where
        S: Into<&'static str>,
        T: serde::Serialize + serde::Deserialize<'a>,
    {
        self.models.insert(model_name.into(), String::new());

        Ok(self)
    }

    pub fn get_total_connections(&self) -> usize {
        self.connections.len()
    }
}

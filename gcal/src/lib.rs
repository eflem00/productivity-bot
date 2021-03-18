use chrono::prelude::*;
use chrono::Duration;
use reqwest::header::AUTHORIZATION;
use std::error::Error;

mod contracts;
pub use contracts::Event;

const BASE_AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/token";
const BASE_CALENDAR_URL: &'static str = "https://www.googleapis.com/calendar/v3";

pub struct Client {
    client: reqwest::blocking::Client,
    client_id: String,
    client_secret: String,
    refresh_token: String,
    access_token: Option<String>,
    expires_at: DateTime<Utc>,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str, refresh_token: &str) -> Client {
        Client {
            client: reqwest::blocking::Client::new(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            refresh_token: refresh_token.to_string(),
            access_token: None,
            expires_at: Utc::now(),
        }
    }

    fn validate_token(&mut self) -> Result<(), Box<dyn Error>> {
        if self.access_token.is_none() || self.expires_at <= Utc::now() {
            println!("refreshing token");
            let url = format!(
                "{}?refresh_token={}&client_id={}&client_secret={}&redirect_uri=urn:ietf:wg:oauth:2.0:oob&grant_type=refresh_token",
                BASE_AUTH_URL,
                self.refresh_token,
                self.client_id,
                self.client_secret
            );
            let resp: contracts::AuthResp = self.client.post(url).body("").send()?.json()?;
            self.access_token = Some(resp.access_token);
            self.expires_at = Utc::now() + Duration::minutes(50);
        } else {
            println!("token is fine");
        }
        Ok(())
    }

    pub fn get_events(
        &mut self,
        time_min: DateTime<Utc>,
        time_max: DateTime<Utc>,
    ) -> Result<Vec<contracts::Event>, Box<dyn Error>> {
        self.validate_token()?;
        let url = format!(
            "{}/calendars/primary/events?timeMin={:?}&timeMax={:?}",
            BASE_CALENDAR_URL, time_min, time_max
        );
        let resp: contracts::EventsResp = self
            .client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.access_token.clone().unwrap()), // TODO: clone?
            )
            .send()?
            .json()?;
        Ok(resp.items)
    }
}

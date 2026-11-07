use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Cancelled,
    Exhausted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub merchant: String,
    pub name: String,
    pub amount: String,
    pub token: String,
    pub interval_ledgers: u32,
    pub created_ledger: u32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub plan_id: String,
    pub subscriber: String,
    pub merchant: String,
    pub cap: String,
    pub total_charged: String,
    pub status: SubscriptionStatus,
    pub start_ledger: u32,
    pub last_charge_ledger: Option<u32>,
    pub next_charge_ledger: Option<u32>,
    pub interval_ledgers: u32,
    pub amount: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_plans: usize,
    pub active_plans: usize,
    pub total_subscriptions: usize,
    pub active_subscriptions: usize,
    pub total_charged_volume: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub plans: Arc<RwLock<HashMap<String, Plan>>>,
    pub subscriptions: Arc<RwLock<HashMap<String, Subscription>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            plans: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_stats(&self) -> Result<Stats, anyhow::Error> {
        let plans = self.plans.read().map_err(|_| anyhow::anyhow!("Failed to acquire plans lock"))?;
        let subscriptions = self.subscriptions.read().map_err(|_| anyhow::anyhow!("Failed to acquire subscriptions lock"))?;

        let total_plans = plans.len();
        let active_plans = plans.values().filter(|p| p.is_active).count();
        let total_subscriptions = subscriptions.len();
        let active_subscriptions = subscriptions
            .values()
            .filter(|s| s.status == SubscriptionStatus::Active)
            .count();

        let mut total_charged_volume: HashMap<String, String> = HashMap::new();
        for sub in subscriptions.values() {
            let current = total_charged_volume
                .get(&sub.token)
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(0);
            let charged = sub.total_charged.parse::<i64>().unwrap_or(0);
            total_charged_volume.insert(sub.token.clone(), (current + charged).to_string());
        }

        Ok(Stats {
            total_plans,
            active_plans,
            total_subscriptions,
            active_subscriptions,
            total_charged_volume,
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

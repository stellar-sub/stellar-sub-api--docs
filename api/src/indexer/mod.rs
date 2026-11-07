use crate::models::{AppState, Plan, Subscription, SubscriptionStatus};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

const HORIZON_URL: &str = "https://horizon-testnet.stellar.org";
const POLL_INTERVAL_SECS: u64 = 10;

#[derive(Debug, Deserialize)]
struct HorizonResponse {
    #[serde(rename = "_embedded")]
    embedded: Embedded,
}

#[derive(Debug, Deserialize)]
struct Embedded {
    records: Vec<Transaction>,
}

#[derive(Debug, Deserialize)]
struct Transaction {
    id: String,
    ledger: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ContractEvent {
    PlanCreated {
        plan_id: String,
        merchant: String,
        name: String,
        amount: String,
        token: String,
        interval_ledgers: u32,
    },
    PlanDeactivated {
        plan_id: String,
    },
    Subscribed {
        subscription_id: String,
        plan_id: String,
        subscriber: String,
        merchant: String,
        cap: String,
        amount: String,
        token: String,
        interval_ledgers: u32,
    },
    Charged {
        subscription_id: String,
        amount: String,
    },
    Cancelled {
        subscription_id: String,
    },
    Paused {
        subscription_id: String,
    },
    Resumed {
        subscription_id: String,
    },
}

pub struct Indexer {
    state: Arc<AppState>,
    client: reqwest::Client,
    last_ledger: u32,
}

impl Indexer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            client: reqwest::Client::new(),
            last_ledger: 0,
        }
    }

    pub async fn start(mut self) {
        info!("Starting indexer with poll interval of {} seconds", POLL_INTERVAL_SECS);

        loop {
            if let Err(e) = self.poll_and_index().await {
                error!("Indexer error: {}", e);
            }

            tokio::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
        }
    }

    async fn poll_and_index(&mut self) -> Result<()> {
        let url = format!(
            "{}/transactions?order=asc&limit=200&cursor={}",
            HORIZON_URL, self.last_ledger
        );

        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .context("Failed to fetch transactions from Horizon")?;

        if !response.status().is_success() {
            warn!("Horizon returned status: {}", response.status());
            return Ok(());
        }

        let horizon_data: HorizonResponse = response
            .json()
            .await
            .context("Failed to parse Horizon response")?;

        let records = horizon_data.embedded.records;
        if records.is_empty() {
            debug!("No new transactions since ledger {}", self.last_ledger);
            return Ok(());
        }

        info!("Processing {} new transactions", records.len());

        for tx in &records {
            if let Err(e) = self.process_transaction(tx).await {
                error!("Failed to process transaction {}: {}", tx.id, e);
            }
            self.last_ledger = tx.ledger;
        }

        Ok(())
    }

    async fn process_transaction(&self, tx: &Transaction) -> Result<()> {
        let events = self.extract_events(tx).await?;

        for event in events {
            if let Err(e) = self.handle_event(event, tx.ledger).await {
                error!("Failed to handle event: {}", e);
            }
        }

        Ok(())
    }

    async fn extract_events(&self, _tx: &Transaction) -> Result<Vec<ContractEvent>> {
        // In a real implementation, this would parse contract events from the transaction
        // For v1, we're simulating event extraction
        Ok(vec![])
    }

    async fn handle_event(&self, event: ContractEvent, ledger: u32) -> Result<()> {
        match event {
            ContractEvent::PlanCreated {
                plan_id,
                merchant,
                name,
                amount,
                token,
                interval_ledgers,
            } => {
                let plan = Plan {
                    id: plan_id.clone(),
                    merchant,
                    name,
                    amount,
                    token,
                    interval_ledgers,
                    created_ledger: ledger,
                    is_active: true,
                };

                let mut plans = self.state.plans.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire plans write lock"))?;
                plans.insert(plan_id, plan);
                info!("Plan created");
            }
            ContractEvent::PlanDeactivated { plan_id } => {
                let mut plans = self.state.plans.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire plans write lock"))?;
                if let Some(plan) = plans.get_mut(&plan_id) {
                    plan.is_active = false;
                    info!("Plan deactivated: {}", plan_id);
                }
            }
            ContractEvent::Subscribed {
                subscription_id,
                plan_id,
                subscriber,
                merchant,
                cap,
                amount,
                token,
                interval_ledgers,
            } => {
                let next_charge = ledger + interval_ledgers;
                let subscription = Subscription {
                    id: subscription_id.clone(),
                    plan_id,
                    subscriber,
                    merchant,
                    cap,
                    total_charged: "0".to_string(),
                    status: SubscriptionStatus::Active,
                    start_ledger: ledger,
                    last_charge_ledger: None,
                    next_charge_ledger: Some(next_charge),
                    interval_ledgers,
                    amount,
                    token,
                };

                let mut subscriptions = self.state.subscriptions.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire subscriptions write lock"))?;
                subscriptions.insert(subscription_id, subscription);
                info!("Subscription created");
            }
            ContractEvent::Charged { subscription_id, amount } => {
                let mut subscriptions = self.state.subscriptions.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire subscriptions write lock"))?;
                
                if let Some(sub) = subscriptions.get_mut(&subscription_id) {
                    let current_charged = sub.total_charged.parse::<i64>().unwrap_or(0);
                    let charge_amount = amount.parse::<i64>().unwrap_or(0);
                    let new_total = current_charged + charge_amount;
                    let cap = sub.cap.parse::<i64>().unwrap_or(0);

                    sub.total_charged = new_total.to_string();
                    sub.last_charge_ledger = Some(ledger);
                    sub.next_charge_ledger = Some(ledger + sub.interval_ledgers);

                    if new_total >= cap {
                        sub.status = SubscriptionStatus::Exhausted;
                        sub.next_charge_ledger = None;
                        info!("Subscription exhausted: {}", subscription_id);
                    } else {
                        info!("Subscription charged: {}", subscription_id);
                    }
                }
            }
            ContractEvent::Cancelled { subscription_id } => {
                let mut subscriptions = self.state.subscriptions.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire subscriptions write lock"))?;
                
                if let Some(sub) = subscriptions.get_mut(&subscription_id) {
                    sub.status = SubscriptionStatus::Cancelled;
                    sub.next_charge_ledger = None;
                    info!("Subscription cancelled: {}", subscription_id);
                }
            }
            ContractEvent::Paused { subscription_id } => {
                let mut subscriptions = self.state.subscriptions.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire subscriptions write lock"))?;
                
                if let Some(sub) = subscriptions.get_mut(&subscription_id) {
                    sub.status = SubscriptionStatus::Paused;
                    info!("Subscription paused: {}", subscription_id);
                }
            }
            ContractEvent::Resumed { subscription_id } => {
                let mut subscriptions = self.state.subscriptions.write()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire subscriptions write lock"))?;
                
                if let Some(sub) = subscriptions.get_mut(&subscription_id) {
                    sub.status = SubscriptionStatus::Active;
                    if sub.next_charge_ledger.is_none() {
                        sub.next_charge_ledger = Some(ledger + sub.interval_ledgers);
                    }
                    info!("Subscription resumed: {}", subscription_id);
                }
            }
        }

        Ok(())
    }
}

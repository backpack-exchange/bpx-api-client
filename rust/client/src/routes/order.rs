use bpx_api_types::order::{CancelOpenOrdersPayload, CancelOrderPayload, ExecuteOrderPayload, Order};

use crate::error::{Error, Result};
use crate::BpxClient;

#[doc(hidden)]
pub const API_ORDER: &str = "/api/v1/order";
#[doc(hidden)]
pub const API_ORDERS: &str = "/api/v1/orders";

impl BpxClient {
    /// Fetches a specific open order by symbol and either order ID or client ID.
    pub async fn get_open_order(&self, symbol: &str, order_id: Option<&str>, client_id: Option<u32>) -> Result<Order> {
        let mut url = format!("{}{}?symbol={}", self.base_url, API_ORDER, symbol);
        if let Some(order_id) = order_id {
            url.push_str(&format!("&orderId={}", order_id));
        } else {
            url.push_str(&format!(
                "&clientId={}",
                client_id.ok_or_else(|| Error::InvalidRequest("either order_id or client_id is required".into()))?
            ));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Executes a new order with the given payload.
    pub async fn execute_order(&self, payload: ExecuteOrderPayload) -> Result<Order> {
        let endpoint = format!("{}{}", self.base_url, API_ORDER);
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    /// Cancels a specific order by symbol and either order ID or client ID.
    pub async fn cancel_order(&self, symbol: &str, order_id: Option<&str>, client_id: Option<u32>) -> Result<Order> {
        let url = format!("{}{}", self.base_url, API_ORDER);
        let payload = CancelOrderPayload {
            symbol: symbol.to_string(),
            order_id: order_id.map(|s| s.to_string()),
            client_id,
        };

        let res = self.delete(url, payload).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves all open orders, optionally filtered by symbol.
    pub async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let mut url = format!("{}{}", self.base_url, API_ORDERS);
        if let Some(s) = symbol {
            url.push_str(&format!("?symbol={s}"));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Cancels all open orders matching the specified payload.
    pub async fn cancel_open_orders(&self, payload: CancelOpenOrdersPayload) -> Result<Vec<Order>> {
        let url = format!("{}{}", self.base_url, API_ORDERS);
        let res = self.delete(url, payload).await?;
        res.json().await.map_err(Into::into)
    }
}

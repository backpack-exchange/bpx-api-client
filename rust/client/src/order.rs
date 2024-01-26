use bpx_api_types::order::{
    CancelOpenOrdersPayload, CancelOrderPayload, ExecuteOrderPayload, Order,
};

use crate::error::{Error, Result};
use crate::BpxClient;

impl BpxClient {
    pub async fn get_open_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        client_id: Option<u32>,
    ) -> Result<Order> {
        let mut url = format!("{}/api/v1/order?symbol={}", self.base_url, symbol);
        if let Some(order_id) = order_id {
            url.push_str(&format!("&orderId={}", order_id));
        } else {
            url.push_str(&format!(
                "&clientId={}",
                client_id.ok_or_else(|| Error::InvalidRequest(
                    "either order_id or client_id is required".to_string()
                ))?
            ));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn execute_order(&self, payload: ExecuteOrderPayload) -> Result<Order> {
        let endpoint = format!("{}/api/v1/order", self.base_url);
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        client_id: Option<u32>,
    ) -> Result<Order> {
        let url = format!("{}/api/v1/order", self.base_url);
        let payload = CancelOrderPayload {
            symbol: symbol.to_string(),
            order_id: order_id.map(|s| s.to_string()),
            client_id,
        };

        let res = self.delete(url, payload).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let mut url = format!("{}/api/v1/orders", self.base_url);
        if let Some(s) = symbol {
            url.push_str(&format!("?symbol={s}"));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn cancel_open_orders(&self, payload: CancelOpenOrdersPayload) -> Result<Vec<Order>> {
        let url = format!("{}/api/v1/orders", self.base_url);
        let res = self.delete(url, payload).await?;
        res.json().await.map_err(Into::into)
    }
}

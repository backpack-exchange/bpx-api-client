use std::{fmt, str::FromStr};

use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerBy {
    LastPrice,
    MarkPrice,
    IndexPrice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerQuantity {
    Percent(Decimal),
    Amount(Decimal),
}

impl<'de> Deserialize<'de> for TriggerQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QtyVisitor;

        impl<'de> Visitor<'de> for QtyVisitor {
            type Value = TriggerQuantity;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#"a string like "12.5%" or "0.01", or a number"#)
            }

            // ---------- JSON string ----------
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                parse_str(v).map_err(serde::de::Error::custom)
            }

            // ---------- JSON numbers ----------
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Decimal::from_f64(v)
                    .ok_or_else(|| serde::de::Error::custom("not a finite number"))
                    .map(TriggerQuantity::Amount)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TriggerQuantity::Amount(Decimal::from(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TriggerQuantity::Amount(Decimal::from(v)))
            }
        }

        deserializer.deserialize_any(QtyVisitor)
    }
}

impl Serialize for TriggerQuantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            match self {
                Self::Percent(percent) => format!("{percent}%"),
                Self::Amount(amount) => format!("{amount}"),
            }
            .as_str(),
        )
    }
}

fn parse_str(s: &str) -> Result<TriggerQuantity, &'static str> {
    if let Some(num) = s.strip_suffix('%') {
        let d = Decimal::from_str(num.trim()).map_err(|_| "invalid percent value")?;
        Ok(TriggerQuantity::Percent(d))
    } else {
        let d = Decimal::from_str(s.trim()).map_err(|_| "invalid decimal value")?;
        Ok(TriggerQuantity::Amount(d))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Option<Decimal>,
    pub executed_quantity: Decimal,
    pub quote_quantity: Option<Decimal>,
    pub executed_quote_quantity: Decimal,
    pub stop_loss_trigger_price: Option<Decimal>,
    pub stop_loss_limit_price: Option<Decimal>,
    pub stop_loss_trigger_by: Option<Decimal>,
    pub take_profit_trigger_price: Option<Decimal>,
    pub take_profit_limit_price: Option<Decimal>,
    pub take_profit_trigger_by: Option<Decimal>,
    pub trigger_by: Option<TriggerBy>,
    pub trigger_price: Option<Decimal>,
    pub trigger_quantity: Option<TriggerQuantity>,
    pub triggered_at: Option<i64>,
    pub time_in_force: TimeInForce,
    pub related_order_id: Option<String>,
    pub self_trade_prevention: SelfTradePrevention,
    pub reduce_only: Option<bool>,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub executed_quantity: Decimal,
    pub executed_quote_quantity: Decimal,
    pub stop_loss_trigger_price: Option<Decimal>,
    pub stop_loss_limit_price: Option<Decimal>,
    pub stop_loss_trigger_by: Option<Decimal>,
    pub take_profit_trigger_price: Option<Decimal>,
    pub take_profit_limit_price: Option<Decimal>,
    pub take_profit_trigger_by: Option<Decimal>,
    pub price: Decimal,
    pub trigger_by: Option<TriggerBy>,
    pub trigger_price: Option<Decimal>,
    pub trigger_quantity: Option<TriggerQuantity>,
    pub triggered_at: Option<i64>,
    pub time_in_force: TimeInForce,
    pub related_order_id: Option<String>,
    pub self_trade_prevention: SelfTradePrevention,
    pub post_only: bool,
    pub reduce_only: Option<bool>,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "orderType")]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SelfTradePrevention {
    #[default]
    RejectTaker,
    RejectMaker,
    RejectBoth,
    Allow,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    Cancelled,
    Expired,
    Filled,
    #[default]
    New,
    PartiallyFilled,
    Triggered,
    TriggerPending,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    #[default]
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOrderPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_redeem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow_repay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention: Option<SelfTradePrevention>,
    pub side: Side,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_quantity: Option<TriggerQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderPayload {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOpenOrdersPayload {
    pub symbol: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use serde_json::json;

    #[test]
    fn both_forms_round_trip() {
        let q: TriggerQuantity = serde_json::from_value(json!("12.5%")).unwrap();
        assert_eq!(q, TriggerQuantity::Percent(dec!(12.5)));

        let q: TriggerQuantity = serde_json::from_value(json!("0.01")).unwrap();
        assert_eq!(q, TriggerQuantity::Amount(dec!(0.01)));
    }

    #[test]
    fn test_trigger_quantity_serialize() {
        let trigger_quantity = TriggerQuantity::Percent(dec!(100));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"100%\"");

        let trigger_quantity = TriggerQuantity::Percent(dec!(75.50));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"75.50%\"");

        let trigger_quantity = TriggerQuantity::Amount(dec!(100));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"100\"");

        let trigger_quantity = TriggerQuantity::Amount(dec!(75.50));
        let trigger_quantity_str = serde_json::to_string(&trigger_quantity).unwrap();
        assert_eq!(trigger_quantity_str, "\"75.50\"");
    }

    #[test]
    fn test_trigger_by_serialize() {
        let trigger_by_last = TriggerBy::LastPrice;
        let trigger_by_last_str = serde_json::to_string(&trigger_by_last).unwrap();
        assert_eq!(trigger_by_last_str, "\"LastPrice\"");

        let trigger_by_mark = TriggerBy::MarkPrice;
        let trigger_by_mark_str = serde_json::to_string(&trigger_by_mark).unwrap();
        assert_eq!(trigger_by_mark_str, "\"MarkPrice\"");

        let trigger_by_index = TriggerBy::IndexPrice;
        let trigger_by_index_str = serde_json::to_string(&trigger_by_index).unwrap();
        assert_eq!(trigger_by_index_str, "\"IndexPrice\"");
    }
}

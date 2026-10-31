// SPDX-License-Identifier: MPL-2.0

use super::*;
use crate::model::{CreditBalances, ProviderCost};

fn parse(data: &Value) -> Result<UsageSnapshot, CommandCodeError> {
    usage::parse(data, None, ProviderIdentity::default(), chrono::Utc::now())
}

#[test]
fn monthly_plan_and_extra_credit_balances_remain_separate() {
    let cost = parse(&credits()).unwrap().provider_cost.unwrap();
    assert_eq!(cost.used, 4.0);
    assert_eq!(
        cost.credit_balances,
        Some(CreditBalances {
            plan: Some(12.5),
            extra: 4.0,
        })
    );
}

#[test]
fn missing_plan_credits_do_not_become_an_invented_zero() {
    let data = json!({"credits":{"purchasedCredits":3,"freeCredits":1}});
    let cost = parse(&data).unwrap().provider_cost.unwrap();
    assert_eq!(
        cost.credit_balances,
        Some(CreditBalances {
            plan: None,
            extra: 4.0,
        })
    );
}

#[test]
fn zero_plan_credits_are_preserved() {
    let mut data = credits();
    data["credits"]["monthlyCredits"] = json!(0);
    let balances = parse(&data)
        .unwrap()
        .provider_cost
        .unwrap()
        .credit_balances
        .unwrap();
    assert_eq!(balances.plan, Some(0.0));
    assert_eq!(balances.extra, 4.0);
}

#[test]
fn invalid_plan_credits_are_not_reported_as_available() {
    for invalid in [json!(null), json!("unknown"), json!(-1)] {
        let mut data = credits();
        data["credits"]["monthlyCredits"] = invalid;
        assert!(matches!(
            parse(&data),
            Err(CommandCodeError::InvalidResponse)
        ));
    }
}

#[test]
fn separate_balances_survive_snapshot_serialization() {
    let snapshot = parse(&credits()).unwrap();
    let serialized = serde_json::to_value(&snapshot).unwrap();
    assert_eq!(
        serialized["provider_cost"]["credit_balances"],
        json!({"plan":12.5,"extra":4.0})
    );
    let restored: UsageSnapshot = serde_json::from_value(serialized).unwrap();
    assert_eq!(restored, snapshot);
}

#[test]
fn legacy_costs_without_separate_balances_still_round_trip() {
    let legacy = json!({"used":30.0,"limit":null,"units":"credits"});
    let restored: ProviderCost = serde_json::from_value(legacy.clone()).unwrap();
    assert!(restored.credit_balances.is_none());
    assert_eq!(serde_json::to_value(restored).unwrap(), legacy);
}

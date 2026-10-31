// SPDX-License-Identifier: MPL-2.0

use super::{Message, card};
use crate::fl;
use crate::model::CreditBalances;
use cosmic::Element;
use cosmic::iced::Length;
use cosmic::iced::widget::{column, row};
use cosmic::widget;

pub(super) fn view(balances: Option<&CreditBalances>) -> Element<'static, Message> {
    let mut content = column![widget::text(fl!("credits-label")).size(15)].spacing(6);
    let Some(rows) = balance_rows(balances) else {
        return card(content.push(widget::text(fl!("credits-unavailable")).size(14)));
    };
    for (label, amount) in rows {
        content = content.push(row![
            widget::text(label).size(14),
            cosmic::iced::widget::Space::new().width(Length::Fill),
            widget::text(amount).size(14),
        ]);
    }
    card(content)
}

fn balance_rows(balances: Option<&CreditBalances>) -> Option<Vec<(String, String)>> {
    let balances = balances?;
    let mut rows = Vec::with_capacity(2);
    if let Some(plan) = balances.plan {
        rows.push((fl!("plan-credits-label"), format!("{plan:.2}")));
    }
    rows.push((fl!("extra-credits-label"), format!("{:.2}", balances.extra)));
    Some(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_and_extra_credits_are_separate_and_rounded_independently() {
        let balances = CreditBalances {
            plan: Some(12.5),
            extra: 0.004,
        };
        assert_eq!(
            balance_rows(Some(&balances)).unwrap(),
            [
                (fl!("plan-credits-label"), "12.50".to_string()),
                (fl!("extra-credits-label"), "0.00".to_string()),
            ]
        );
    }

    #[test]
    fn zero_extra_credits_keep_two_decimal_places() {
        let balances = CreditBalances {
            plan: Some(20.0),
            extra: 0.0,
        };
        assert_eq!(balance_rows(Some(&balances)).unwrap()[1].1, "0.00");
    }

    #[test]
    fn missing_plan_credits_do_not_become_an_invented_zero() {
        let balances = CreditBalances {
            plan: None,
            extra: 4.0,
        };
        assert_eq!(
            balance_rows(Some(&balances)).unwrap(),
            [(fl!("extra-credits-label"), "4.00".to_string())]
        );
    }

    #[test]
    fn legacy_combined_balances_are_not_relabelled_as_extra_credits() {
        assert!(balance_rows(None).is_none());
    }
}

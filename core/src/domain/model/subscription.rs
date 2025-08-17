use serde::{Deserialize, Serialize};

use crate::{
    domain::model::{subscription_id::SubscriptionId, year_month::YearMonth},
    error::{DomainError, DomainResult},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SubscriptionName(String);

impl SubscriptionName {
    pub fn new(name: String) -> DomainResult<Self> {
        if name.trim().is_empty() {
            return Err(DomainError::EmptySubscriptionName);
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Amount {
    value: u32,
    currency: Currency,
}

impl Amount {
    pub fn new(value: u32, currency: Currency) -> Self {
        Self { value, currency }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Currency {
    JPY,
    USD,
    EUR,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum BillingCycle {
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PaymentMethod {
    CreditCard,
    PayPal,
    ApplePay,
    GooglePay,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PaymentDay {
    MonthlyFirst,
    MonthlyLast,
    MonthlySpecific(u8),
    YearlySpecific { month: u8, day: u8 },
}

impl PaymentDay {
    pub fn monthly_specific(day: u8) -> DomainResult<Self> {
        if !(1..=31).contains(&day) {
            return Err(DomainError::InvalidPaymentDay(day));
        }
        Ok(Self::MonthlySpecific(day))
    }

    pub fn yearly_specific(month: u8, day: u8) -> DomainResult<Self> {
        if !(1..=12).contains(&month) {
            return Err(DomainError::InvalidMonth(month));
        }
        if !(1..=31).contains(&day) {
            return Err(DomainError::InvalidDay(day));
        }
        Ok(Self::YearlySpecific { month, day })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Subscription {
    id: SubscriptionId,
    name: SubscriptionName,
    amount: Amount,
    billing_cycle: BillingCycle,
    auto_renewal: bool,
    payment_day: PaymentDay,
    payment_method: PaymentMethod,
}

impl Subscription {
    pub fn new(
        id: SubscriptionId,
        name: SubscriptionName,
        amount: Amount,
        billing_cycle: BillingCycle,
        auto_renewal: bool,
        payment_day: PaymentDay,
        payment_method: PaymentMethod,
    ) -> Self {
        Self {
            id,
            name,
            amount,
            billing_cycle,
            auto_renewal,
            payment_day,
            payment_method,
        }
    }

    pub fn id(&self) -> &SubscriptionId {
        &self.id
    }

    pub fn name(&self) -> &SubscriptionName {
        &self.name
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn billing_cycle(&self) -> &BillingCycle {
        &self.billing_cycle
    }

    pub fn auto_renewal(&self) -> bool {
        self.auto_renewal
    }

    pub fn payment_day(&self) -> &PaymentDay {
        &self.payment_day
    }

    pub fn payment_method(&self) -> &PaymentMethod {
        &self.payment_method
    }

    pub fn change_amount(&self, new_amount: Amount) -> Self {
        Self {
            amount: new_amount,
            ..self.clone()
        }
    }

    pub fn toggle_auto_renewal(&self) -> Self {
        Self {
            auto_renewal: !self.auto_renewal,
            ..self.clone()
        }
    }

    pub fn calculate_monthly_amount(&self, year_month: &YearMonth) -> u32 {
        match &self.billing_cycle {
            BillingCycle::Monthly => self.amount.value(),
            BillingCycle::Yearly => {
                if self.is_payment_month(year_month) {
                    self.amount.value()
                } else {
                    0
                }
            }
        }
    }

    fn is_payment_month(&self, year_month: &YearMonth) -> bool {
        match &self.payment_day {
            PaymentDay::MonthlyFirst | PaymentDay::MonthlyLast | PaymentDay::MonthlySpecific(_) => {
                self.billing_cycle == BillingCycle::Monthly
            }
            PaymentDay::YearlySpecific {
                month: payment_month,
                day: _,
            } => {
                self.billing_cycle == BillingCycle::Yearly
                    && year_month.month().value() == *payment_month
            }
        }
    }
}

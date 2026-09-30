//! Per-attempt process and explicit call-budget reservations.

use std::sync::atomic::Ordering;

use super::{Cancel, error};

/// One caller's immutable limits against the retained process counters.
#[derive(Clone, Debug)]
pub(crate) struct ProcessBudget {
    pub(crate) budget: crate::engine::budget::SendBudget,
    pub(crate) requests: Option<u64>,
    pub(crate) estimated: Option<u64>,
}

/// Reservations for one attempted send. Both counters are committed only
/// after the usage mark; dropping either uncommitted reservation refunds it.
pub(crate) struct SendReservations(
    Option<crate::engine::budget::SendReservation>,
    Option<crate::engine::budget::SendReservation>,
    Option<crate::engine::budget::EstimatedReservation>,
);

impl SendReservations {
    pub(crate) fn commit(self) {
        if let Some(reservation) = self.0 {
            reservation.commit();
        }
        if let Some(reservation) = self.1 {
            reservation.commit();
        }
        if let Some(reservation) = self.2 {
            reservation.commit();
        }
    }
}

/// The estimated input limit named by `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL`.
pub(crate) fn estimated_total(text: Option<String>) -> Result<Option<u64>, &'static str> {
    text.map(|text| {
        text.parse::<u64>()
            .ok()
            .filter(|_| text.bytes().all(|byte| byte.is_ascii_digit()))
            .ok_or("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL takes a whole number of 0 or more")
    })
    .transpose()
}

impl Cancel<'_> {
    pub(crate) fn with_send_budget(
        &self,
        send_budget: Option<(crate::engine::budget::SendBudget, Option<u64>)>,
    ) -> Self {
        Self {
            send_budget,
            ..self.clone()
        }
    }

    pub(crate) fn with_process_budget(&self, process_budget: Option<ProcessBudget>) -> Self {
        Self {
            process_budget,
            ..self.clone()
        }
    }

    /// Only a zero limit is certainly spent without reserving an attempt.
    pub(crate) fn has_zero_send_limit(&self) -> bool {
        matches!(self.send_budget, Some((_, Some(0))))
            || self
                .process_budget
                .as_ref()
                .is_some_and(|selected| selected.requests == Some(0))
    }

    pub(crate) fn reserve_send(
        &self,
        last_status: Option<u16>,
        body_bytes: usize,
    ) -> Result<Option<SendReservations>, error::Error> {
        let reserve = |selected: &Option<(crate::engine::budget::SendBudget, Option<u64>)>| {
            selected
                .as_ref()
                .map(|(budget, limit)| budget.reserve(*limit, last_status))
                .transpose()
                .map_err(|denial| match denial {
                    crate::core::SendBudgetDenial::BeforeFirstSend
                        if self.sent_any.load(Ordering::Acquire) =>
                    {
                        error::Error::SendBudgetAdditional
                    }
                    crate::core::SendBudgetDenial::BeforeFirstSend => error::Error::SendBudgetFirst,
                    crate::core::SendBudgetDenial::BeforeAdditionalSend => {
                        error::Error::SendBudgetAdditional
                    }
                    crate::core::SendBudgetDenial::BeforeRetry { last_status } => {
                        error::Error::SendBudgetRetry(last_status)
                    }
                })
        };
        let process = reserve(
            &self
                .process_budget
                .as_ref()
                .map(|selected| (selected.budget.clone(), selected.requests)),
        )?;
        let explicit = reserve(&self.send_budget)?;
        let estimated = self
            .process_budget
            .as_ref()
            .map(|selected| {
                selected
                    .budget
                    .reserve_estimated(selected.estimated, body_bytes)
                    .map_err(|()| self.estimated_denial(selected.estimated, last_status))
            })
            .transpose()?;
        if process.is_none() && explicit.is_none() && estimated.is_none() {
            Ok(None)
        } else {
            Ok(Some(SendReservations(process, explicit, estimated)))
        }
    }
    fn estimated_denial(&self, limit: Option<u64>, last_status: Option<u16>) -> error::Error {
        let Some(limit) = limit else {
            return error::Error::Usage("estimated input admission cannot be counted");
        };
        let reason = match (last_status, self.sent_any.load(Ordering::Acquire)) {
            (Some(last_status), _) => {
                crate::core::EstimatedInputDenial::Retry { limit, last_status }
            }
            (None, true) => crate::core::EstimatedInputDenial::AdditionalRequest { limit },
            (None, false) => crate::core::EstimatedInputDenial::InitialRequest { limit },
        };
        error::Error::EstimatedInput(reason)
    }
}

#[cfg(test)]
#[test]
fn unstarted_estimate_and_request_refund_together() {
    let budget = crate::engine::budget::SendBudget::new();
    let cancel = super::Cancel::default().with_process_budget(Some(ProcessBudget {
        budget: budget.clone(),
        requests: Some(2),
        estimated: Some(2),
    }));
    let pending = cancel
        .reserve_send(None, 2)
        .expect("first admission")
        .expect("reservations");
    drop(pending); // The attempt never reached the usage mark or transport.
    cancel
        .reserve_send(None, 2)
        .expect("refund admits same body")
        .expect("reservations")
        .commit();
    assert!(matches!(
        cancel.reserve_send(None, 2),
        Err(super::error::Error::EstimatedInput(
            crate::core::EstimatedInputDenial::InitialRequest { limit: 2 }
        ))
    ));
    budget
        .reserve(Some(2), None)
        .expect("the denied estimate refunded its request slot");
}

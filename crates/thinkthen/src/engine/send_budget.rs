//! Per-attempt process and explicit call-budget reservations.

use std::sync::atomic::Ordering;

use super::{Cancel, error};

/// Reservations for one attempted send. Both counters are committed only
/// after the usage mark; dropping either uncommitted reservation refunds it.
pub(crate) struct SendReservations(
    Option<crate::public::SendReservation>,
    Option<crate::public::SendReservation>,
);

impl SendReservations {
    pub(crate) fn commit(self) {
        if let Some(reservation) = self.0 {
            reservation.commit();
        }
        if let Some(reservation) = self.1 {
            reservation.commit();
        }
    }
}

impl Cancel<'_> {
    pub(crate) fn with_send_budget(
        &self,
        send_budget: Option<(crate::public::SendBudget, Option<u64>)>,
    ) -> Self {
        Self {
            send_budget,
            ..self.clone()
        }
    }

    pub(crate) fn with_process_budget(
        &self,
        process_budget: Option<(crate::public::SendBudget, Option<u64>)>,
    ) -> Self {
        Self {
            process_budget,
            ..self.clone()
        }
    }

    /// Only a zero limit is certainly spent without reserving an attempt.
    pub(crate) fn has_zero_send_limit(&self) -> bool {
        matches!(self.send_budget, Some((_, Some(0))))
            || matches!(self.process_budget, Some((_, Some(0))))
    }

    pub(crate) fn reserve_send(
        &self,
        last_status: Option<u16>,
    ) -> Result<Option<SendReservations>, error::Error> {
        let reserve = |selected: &Option<(crate::public::SendBudget, Option<u64>)>| {
            selected
                .as_ref()
                .map(|(budget, limit)| budget.reserve(*limit, last_status))
                .transpose()
                .map_err(|denial| match denial {
                    crate::public::SendBudgetDenial::BeforeFirstSend
                        if self.sent_any.load(Ordering::Acquire) =>
                    {
                        error::Error::SendBudgetAdditional
                    }
                    crate::public::SendBudgetDenial::BeforeFirstSend => {
                        error::Error::SendBudgetFirst
                    }
                    crate::public::SendBudgetDenial::BeforeAdditionalSend => {
                        error::Error::SendBudgetAdditional
                    }
                    crate::public::SendBudgetDenial::BeforeRetry { last_status } => {
                        error::Error::SendBudgetRetry(last_status)
                    }
                })
        };
        let process = reserve(&self.process_budget)?;
        let explicit = reserve(&self.send_budget)?;
        if process.is_none() && explicit.is_none() {
            Ok(None)
        } else {
            Ok(Some(SendReservations(process, explicit)))
        }
    }
}

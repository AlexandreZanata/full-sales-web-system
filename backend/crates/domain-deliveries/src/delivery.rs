use domain_identity::UserId;
use domain_media::FileId;
use domain_orders::OrderId;
use domain_shared::TenantId;

use crate::delivery_id::DeliveryId;
use crate::delivery_status::DeliveryStatus;
use crate::error::DeliveryError;

pub struct DeliveryCreateInput {
    pub id: DeliveryId,
    pub tenant_id: TenantId,
    pub order_id: OrderId,
    pub driver_id: UserId,
}

pub struct ConfirmDeliveryInput {
    pub proof_file_id: Option<FileId>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub received_by_name: Option<String>,
}

/// Physical fulfillment of an approved order — driver, proof, geo.
#[derive(Debug, Clone)]
pub struct Delivery {
    id: DeliveryId,
    tenant_id: TenantId,
    order_id: OrderId,
    driver_id: UserId,
    status: DeliveryStatus,
    proof_file_id: Option<FileId>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    received_by_name: Option<String>,
}

impl Delivery {
    pub fn create(input: DeliveryCreateInput) -> Self {
        Self {
            id: input.id,
            tenant_id: input.tenant_id,
            order_id: input.order_id,
            driver_id: input.driver_id,
            status: DeliveryStatus::Waiting,
            proof_file_id: None,
            latitude: None,
            longitude: None,
            received_by_name: None,
        }
    }

    pub fn id(&self) -> DeliveryId {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn driver_id(&self) -> UserId {
        self.driver_id
    }

    pub fn status(&self) -> DeliveryStatus {
        self.status
    }

    pub fn proof_file_id(&self) -> Option<FileId> {
        self.proof_file_id
    }

    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    pub fn longitude(&self) -> Option<f64> {
        self.longitude
    }

    pub fn received_by_name(&self) -> Option<&str> {
        self.received_by_name.as_deref()
    }

    pub fn assign_driver(mut self, driver_id: UserId) -> Result<Self, DeliveryError> {
        if self.status != DeliveryStatus::Waiting {
            return Err(DeliveryError::InvalidTransition {
                from: self.status,
                to: self.status,
            });
        }
        self.driver_id = driver_id;
        Ok(self)
    }

    pub fn start_transit(mut self, acting_driver: UserId) -> Result<Self, DeliveryError> {
        ensure_assigned_driver(self.driver_id, acting_driver)?;
        self.transition_to(DeliveryStatus::InTransit)?;
        Ok(self)
    }

    pub fn confirm(
        mut self,
        input: ConfirmDeliveryInput,
        acting_driver: UserId,
    ) -> Result<Self, DeliveryError> {
        ensure_assigned_driver(self.driver_id, acting_driver)?;
        if input.proof_file_id.is_none() {
            return Err(DeliveryError::ProofRequired);
        }
        self.transition_to(DeliveryStatus::Delivered)?;
        self.proof_file_id = input.proof_file_id;
        self.latitude = input.latitude;
        self.longitude = input.longitude;
        self.received_by_name = input
            .received_by_name
            .map(|n| n.trim().to_owned())
            .filter(|n| !n.is_empty());
        Ok(self)
    }

    pub fn restore(
        id: DeliveryId,
        tenant_id: TenantId,
        order_id: OrderId,
        driver_id: UserId,
        status: DeliveryStatus,
        proof_file_id: Option<FileId>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        received_by_name: Option<String>,
    ) -> Self {
        Self {
            id,
            tenant_id,
            order_id,
            driver_id,
            status,
            proof_file_id,
            latitude,
            longitude,
            received_by_name,
        }
    }

    fn transition_to(&mut self, target: DeliveryStatus) -> Result<(), DeliveryError> {
        if !can_transition(self.status, target) {
            return Err(DeliveryError::InvalidTransition {
                from: self.status,
                to: target,
            });
        }
        self.status = target;
        Ok(())
    }
}

fn ensure_assigned_driver(assigned: UserId, acting: UserId) -> Result<(), DeliveryError> {
    if assigned != acting {
        return Err(DeliveryError::DriverNotAssigned);
    }
    Ok(())
}

fn can_transition(from: DeliveryStatus, to: DeliveryStatus) -> bool {
    matches!(
        (from, to),
        (DeliveryStatus::Waiting, DeliveryStatus::InTransit)
            | (DeliveryStatus::InTransit, DeliveryStatus::Delivered)
            | (DeliveryStatus::Waiting, DeliveryStatus::Failed)
            | (DeliveryStatus::InTransit, DeliveryStatus::Failed)
    )
}

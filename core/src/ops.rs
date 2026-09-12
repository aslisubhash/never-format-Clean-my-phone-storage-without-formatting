use crate::types::{OpKind, OpState, Operation, OperationItem};
use chrono::Utc;
use uuid::Uuid;

pub fn new_operation(
    kind: OpKind,
    device_id: &str,
    dry_run: bool,
    items: Vec<OperationItem>,
) -> Operation {
    let now = Utc::now();
    Operation {
        id: Uuid::new_v4(),
        kind,
        state: OpState::Planned,
        device_id: device_id.to_string(),
        created_at: now,
        updated_at: now,
        message: "Planned".into(),
        dry_run,
        items,
    }
}

pub fn advance(op: &mut Operation, state: OpState, message: impl Into<String>) {
    op.state = state;
    op.message = message.into();
    op.updated_at = Utc::now();
}

pub fn can_delete(op: &Operation) -> bool {
    !op.dry_run
        && matches!(
            op.state,
            OpState::AwaitingConfirmation | OpState::Verifying | OpState::Completed
        )
        && op.items.iter().any(|i| i.eligible_for_delete)
}

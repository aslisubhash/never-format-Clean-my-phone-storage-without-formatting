use crate::types::{Operation, OpState, Report};
use chrono::Utc;
use uuid::Uuid;

pub fn from_operation(op: &Operation) -> Report {
    let success = matches!(op.state, OpState::Completed);
    let mut body = String::new();
    body.push_str(&format!("Operation: {:?}\n", op.kind));
    body.push_str(&format!("State: {:?}\n", op.state));
    body.push_str(&format!("Device: {}\n", op.device_id));
    body.push_str(&format!("Dry-run: {}\n", op.dry_run));
    body.push_str(&format!("Message: {}\n\n", op.message));
    body.push_str("Items:\n");
    for item in &op.items {
        body.push_str(&format!(
            "- {} → {:?} | size={} eligible={} deleted={} err={:?}\n",
            item.source_path,
            item.dest_path,
            item.size_bytes,
            item.eligible_for_delete,
            item.deleted,
            item.error
        ));
    }

    Report {
        id: Uuid::new_v4(),
        operation_id: op.id,
        title: format!("{:?} — {:?}", op.kind, op.state),
        body,
        created_at: Utc::now(),
        success,
    }
}

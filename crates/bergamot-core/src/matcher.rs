use crate::descriptor::{ActionDescriptor, DragContext};

pub fn available_actions(
    actions: &[ActionDescriptor],
    context: &DragContext,
) -> Vec<ActionDescriptor> {
    actions
        .iter()
        .filter(|action| action.matches(context))
        .cloned()
        .collect()
}

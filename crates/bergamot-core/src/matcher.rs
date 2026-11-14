use crate::descriptor::{ActionDescriptor, DragContext};

pub struct ActionMatcher;

impl ActionMatcher {
    pub fn available_actions(
        actions: &[ActionDescriptor],
        context: &DragContext,
    ) -> Vec<ActionDescriptor> {
        available_actions(actions, context)
    }
}

pub fn available_actions(
    actions: &[ActionDescriptor],
    context: &DragContext,
) -> Vec<ActionDescriptor> {
    let mut matched: Vec<ActionDescriptor> = actions
        .iter()
        .filter(|action| action.matches(context))
        .cloned()
        .collect();
    matched.sort_by(|a, b| {
        a.sort_order
            .unwrap_or(0)
            .cmp(&b.sort_order.unwrap_or(0))
            .then_with(|| a.label.cmp(&b.label))
            .then_with(|| a.id.cmp(&b.id))
    });
    matched
}

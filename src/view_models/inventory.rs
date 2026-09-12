pub(crate) struct InventoryOverlayView {
    pub(crate) title: &'static str,
    pub(crate) subtitle: &'static str,
    pub(crate) held_title: &'static str,
    pub(crate) detail_title: &'static str,
    pub(crate) uses_title: &'static str,
    pub(crate) sort_text: String,
    pub(crate) empty_text: String,
    pub(crate) page_text: Option<String>,
    pub(crate) items: Vec<InventoryItemView>,
    pub(crate) detail: Option<InventoryDetailView>,
    pub(crate) footer_text: &'static str,
    pub(crate) use_label: &'static str,
    pub(crate) previous_label: &'static str,
    pub(crate) next_label: &'static str,
}

pub(crate) struct InventoryItemView {
    pub(crate) item_id: String,
    pub(crate) title: String,
    pub(crate) meta: String,
    pub(crate) selected: bool,
}

pub(crate) struct InventoryDetailView {
    pub(crate) item_id: String,
    pub(crate) title: String,
    pub(crate) quantity_text: String,
    pub(crate) category_text: String,
    pub(crate) description: String,
    pub(crate) uses_text: String,
    pub(crate) action_text: String,
    pub(crate) can_use: bool,
}

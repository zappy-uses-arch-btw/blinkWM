use crate::wm::Layout;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: usize,
    #[allow(dead_code)]
    pub name: String,
    pub layout: Layout,
}

impl Workspace {
    pub fn new(id: usize, name: Option<String>) -> Self {
        Self {
            id,
            name: name.unwrap_or_else(|| id.to_string()),
            layout: Layout::Vertical,
        }
    }
}

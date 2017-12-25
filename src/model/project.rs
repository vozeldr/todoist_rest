#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    /// Project identifier
    id: Option<u32>,
    /// Project name
    name: String,
    /// Project position in the list of projects (read-only)
    order: Option<u32>,
    /// Value from 1 to 4 for the project indentation level (read-only)
    indent: Option<u32>,
    /// The number of project comments
    comment_count: Option<u32>
}

impl Project {
    pub fn create(name: &str) -> Project {
        Project {
            id: None,
            name: String::from(name),
            order: None,
            indent: None,
            comment_count: None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn order(&self) -> &Option<u32> {
        &self.order
    }

    pub fn indent(&self) -> &Option<u32> {
        &self.indent
    }

    pub fn comment_count(&self) -> &Option<u32> {
        &self.comment_count
    }
}
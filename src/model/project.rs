//! # Project
//!
//! Module containing project-related structures and utilities.

/// Data model for a project that tasks can be grouped into.
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
    /// Creates a new project with the given name.
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

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use model::project::Project;

    #[test]
    fn create_and_serialize_project() {
        let new_project = Project::create("Test Project");
        let json = serde_json::to_string(&new_project).unwrap();
        println!("{}", json);
        assert!(json.contains("\"name\":\"Test Project\""));
    }

    #[test]
    fn deserialize_project() {
        let json = r#"
            {
                "id": 1234,
                "name": "Movies to watch",
                "comment_count": 0,
                "order": 1,
                "indent": 1
            }
        "#;

        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.name(), "Movies to watch");
        assert_eq!(project.id().unwrap(), 1234);
        assert_eq!(project.comment_count().unwrap(), 0);
        assert_eq!(project.order().unwrap(), 1);
        assert_eq!(project.indent().unwrap(), 1);
    }
}

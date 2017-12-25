#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod model;

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use model::project::Project;
    use model::task::Task;

    #[test]
    fn create_and_serialize_project() {
        let new_project = Project::create("Test Project");
        let json = serde_json::to_string(&new_project).unwrap();
        println!("{}", json);
        assert!(json.contains("\"name\":\"Test Project\""));
    }

    #[test]
    fn create_and_serialize_task() {
        let new_task = Task::create("Test Task");
        let json = serde_json::to_string(&new_task).unwrap();
        println!("{}", json);
        assert!(json.contains("\"content\":\"Test Task\""));
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

    #[test]
    fn deserialize_task() {
        let json = r#"
{
    "comment_count": 10,
    "completed": true,
    "content": "My task",
    "due": {
        "date": "2016-09-01",
        "recurring": true,
        "datetime": "2016-09-01T09:00:00Z",
        "string": "tomorrow at 12",
        "timezone": "Europe/Moscow"
    },
    "id": 1234,
    "indent": 1,
    "label_ids": [
        124,
        125,
        128
    ],
    "order": 123,
    "priority": 1,
    "project_id": 2345,
    "url": "https://todoist.com/showTask?id=12345&sync_id=56789"
}
        "#;

        let task: Task = serde_json::from_str(json).unwrap();
        println!("{:?}", task);
        println!("{}", serde_json::to_string(&task).unwrap());
    }
}

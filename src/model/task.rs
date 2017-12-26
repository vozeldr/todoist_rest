//! # Task
//!
//! Module containing task-related structures and utilities.

use serde::ser::{Serialize, Serializer, SerializeStruct};

/// Data model for information about when a task is due.
#[derive(Deserialize, Debug, Clone)]
pub struct Due {
    /// Human defined date in arbitrary format
    string: String,
    /// Date in format YYYY-MM-DD corrected to user’s timezone
    date: Option<String>,
    /// Only returned if exact due time set (i.e. it’s not a whole-day task), date and time in
    /// RFC3339 format in UTC
    datetime: Option<String>,
    /// Only returned if exact due time set, user’s timezone definition either in tzdata-compatible
    /// format (“Europe/Berlin”) or as a string specifying east of UTC offset as “UTC±HH:MM”
    /// (i.e. “UTC-01:00”)
    timezone: Option<String>
}

impl Due {
    /// Creates a new instance of information about when a task is due.
    ///
    /// The `string` is in arbitrary format that will be parsed by the server
    /// (i.e. "tomorrow at noon"). The `date`, `datetime` and `timezone` will be set to `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let due = Due::create("tomorrow at noon");
    /// assert_eq!(due.string(), "tomorrow at noon");
    /// ```
    pub fn create(string: &str) -> Due {
        Due {
            string: String::from(string),
            date: None,
            datetime: None,
            timezone: None
        }
    }

    /// Sets the human-defined information about when the task is due.
    ///
    /// The `date`, `datetime` and `timezone` will be set to `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_string("monday");
    /// assert_eq!(due.string(), "monday");
    /// ```
    pub fn set_string(&mut self, string: &str) {
        self.string = String::from(string);
        self.date = None;
        self.datetime = None;
        self.timezone = None;
    }

    /// Sets the exact date when the task is due (YYYY-MM-DD format).
    ///
    /// The `string` will also be set to the date. The `datetime` and `timezone` will be set to
    /// `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_date("2017-12-25");
    /// assert_eq!(due.string(), "2017-12-25");
    /// assert_eq!(due.date(), Some(String::from("2017-12-25")));
    /// assert_eq!(due.datetime(), None);
    /// ```
    pub fn set_date(&mut self, date: &str) {
        self.string = String::from(date);
        self.date = Some(String::from(date));
        self.datetime = None;
        self.timezone = None;
    }

    /// Sets the date and time when the task is due
    /// ([RFC3339 format in UTC](https://tools.ietf.org/html/rfc3339#section-5.6)).
    ///
    /// The `string` will also be set to the datetime. The `datetime` and `timezone` will be set to
    /// `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_datetime("2017-12-25T12:00:00Z");
    /// assert_eq!(due.string(), "2017-12-25T12:00:00Z");
    /// assert_eq!(due.datetime(), Some(String::from("2017-12-25T12:00:00Z")));
    /// assert_eq!(due.date(), None);
    /// ```
    pub fn set_datetime(&mut self, datetime: &str) {
        self.string = String::from(datetime);
        self.date = None;
        self.datetime = Some(String::from(datetime));
        self.timezone = None;
    }

    /// Gets the human-defined due information.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let due = Due::create("tomorrow at noon");
    /// assert_eq!(due.string(), "tomorrow at noon");
    /// ```
    pub fn string(&self) -> &str {
        &self.string
    }

    /// Gets the date when the task is due.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_date("2017-12-25");
    /// assert_eq!(due.date(), Some(String::from("2017-12-25")));
    /// ```
    pub fn date(&self) -> Option<String> {
        self.date.clone()
    }

    /// Gets the date and time when the task is due.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Due;
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_datetime("2017-12-25T12:00:00Z");
    /// assert_eq!(due.datetime(), Some(String::from("2017-12-25T12:00:00Z")));
    /// ```
    pub fn datetime(&self) -> Option<String> {
        self.datetime.clone()
    }
}

/// Data model for a task.
#[derive(Deserialize, Debug)]
pub struct Task {
    /// Task identifier
    id: Option<u32>,
    /// The task's project identifier (read-only)
    project_id: Option<u32>,
    /// The task content
    content: String,
    /// Flag to mark completed tasks
    completed: bool,
    /// Array of label identifiers associated with the task
    label_ids: Vec<u32>,
    /// Position of the task within the project (read-only)
    order: Option<u32>,
    /// Task indentation level from 1 to 5 (read-only)
    indent: Option<u32>,
    /// Task priority from 1 (normal) to 4 (urgent)
    priority: u32,
    /// Object representing the task due date/time
    due: Option<Due>,
    /// URL to access this task in Todoist web interface
    url: Option<String>,
    /// Number of task comments
    comment_count: Option<u32>
}

impl Task {
    /// Creates a new task.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.content(), "Test Task");
    /// ```
    pub fn create(content: &str) -> Task {
        Task {
            id: None,
            project_id: None,
            content: String::from(content),
            completed: false,
            label_ids: vec![],
            order: None,
            indent: None,
            priority: 1,
            due: None,
            url: None,
            comment_count: None
        }
    }

    /// Sets the information about when the task is due.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::{Task, Due};
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_date("2017-12-25");
    /// let mut task = Task::create("Test Task");
    /// task.set_due(Some(due));
    /// assert_eq!(task.due().unwrap().date(), Some(String::from("2017-12-25")));
    /// ```
    pub fn set_due(&mut self, due: Option<Due>) {
        self.due = due;
    }

    /// Sets the priority for the task from 1 (normal) to 4 (urgent).
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.set_priority(3);
    /// assert_eq!(task.priority(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the provided value is not in the range of 1 to 4.
    pub fn set_priority(&mut self, priority: u32) {
        match priority {
            1...4 => self.priority = priority,
            _ => panic!("The priority must be a value from 1 and 4.")
        };
    }

    /// Removes the association of a label from the task.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.add_label_id(10);
    /// task.add_label_id(4);
    /// task.add_label_id(1);
    /// assert_eq!(task.label_ids(), [10, 4, 1]);
    /// task.remove_label_id(4);
    /// assert_eq!(task.label_ids(), [10, 1]);
    /// ```
    pub fn remove_label_id(&mut self, label_id: u32) {
        self.label_ids.retain(|&id| id != label_id);
    }

    /// Associates a label to the task.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.add_label_id(10);
    /// task.add_label_id(4);
    /// task.add_label_id(1);
    /// assert_eq!(task.label_ids(), [10, 4, 1]);
    /// ```
    pub fn add_label_id(&mut self, label_id: u32) {
        self.label_ids.push(label_id);
    }

    /// Sets the content of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.set_content("New Task Name");
    /// assert_eq!(task.content(), "New Task Name");
    /// ```
    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    /// Sets whether or not the task is completed.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.set_completed(true);
    /// assert!(task.completed());
    /// ```
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    /// Gets the task identifier.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.id(), &None);
    /// ```
    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    /// Gets the identifier for the project that task associated with.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.project_id(), &None);
    /// ```
    pub fn project_id(&self) -> &Option<u32> {
        &self.project_id
    }

    /// Gets the task's content.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.content(), "Test Task");
    /// ```
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Gets whether the task is completed or not.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.completed(), false);
    /// ```
    pub fn completed(&self) -> bool {
        self.completed
    }

    /// Gets the identifiers of the labels associated with the task.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.add_label_id(10);
    /// task.add_label_id(4);
    /// task.add_label_id(1);
    /// assert_eq!(task.label_ids(), [10, 4, 1]);
    /// ```
    pub fn label_ids(&self) -> Vec<u32>{
        self.label_ids.clone()
    }

    /// Gets the order of the task with a list of tasks.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.order(), &None);
    /// ```
    pub fn order(&self) -> &Option<u32>{
        &self.order
    }

    /// Gets the indentation level for the task in a list of tasks.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.indent(), &None);
    /// ```
    pub fn indent(&self) -> &Option<u32>{
        &self.indent
    }

    /// Gets the priority of the task from 1 to 4.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let mut task = Task::create("Test Task");
    /// task.set_priority(3);
    /// assert_eq!(task.priority(), 3);
    /// ```
    pub fn priority(&self) -> u32{
        self.priority
    }

    /// Gets information about when the task is due.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::{Task, Due};
    ///
    /// let mut due = Due::create("tomorrow at noon");
    /// due.set_date("2017-12-25");
    /// let mut task = Task::create("Test Task");
    /// task.set_due(Some(due));
    /// assert_eq!(task.due().unwrap().date(), Some(String::from("2017-12-25")));
    /// ```
    pub fn due(&self) -> Option<Due> {
        self.due.clone()
    }

    /// Gets the URL on the Todoist site where the full task can be viewed.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.url(), &None);
    /// ```
    pub fn url(&self) -> &Option<String> {
        &self.url
    }

    /// Gets the number of comments associated with the task.
    ///
    /// # Example
    ///
    /// ```
    /// use todoist_rest::model::task::Task;
    ///
    /// let task = Task::create("Test Task");
    /// assert_eq!(task.comment_count(), &None);
    /// ```
    pub fn comment_count(&self) -> &Option<u32> {
        &self.comment_count
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut len = 5;

        match &self.due {
            &Some(ref due) => {
                match &due.datetime {
                    &Some(_) => len = 6,
                    &None => {
                        match &due.date {
                            &Some(_) => len = 6,
                            &None => len = 7
                        }
                    }
                }
            },
            &None => ()
        };

        let mut state = serializer.serialize_struct("Task", len)?;

        state.serialize_field("content", &self.content)?;
        state.serialize_field("project_id", &self.project_id)?;
        state.serialize_field("order", &self.order)?;
        state.serialize_field("label_ids", &self.label_ids)?;
        state.serialize_field("priority", &self.priority)?;

        match &self.due {
            &Some(ref due) => {
                match &due.datetime {
                    &Some(ref datetime) => {
                        state.serialize_field("due_datetime", datetime)?;
                    },
                    &None => {
                        match &due.date {
                            &Some(ref date) => {
                                state.serialize_field("due_date", date)?;
                            },
                            &None => {
                                state.serialize_field("due_string", due.string())?;
                                state.serialize_field("due_lang", "en")?;
                            }
                        }
                    }
                }
            },
            &None => ()
        };

        state.end()
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use model::task::Task;
    use model::task::Due;

    #[test]
    fn create_due() {
        let due = Due::create("tomorrow at noon");
        assert_eq!(due.string(), "tomorrow at noon");
    }

    #[test]
    fn set_due_date() {
        let mut due = Due::create("tomorrow at noon");
        due.set_date("2017-12-25");
        assert_eq!(due.date(), Some(String::from("2017-12-25")));
        assert_eq!(due.string(), "2017-12-25");
    }

    #[test]
    fn set_task_due_date() {
        let mut due = Due::create("tomorrow at noon");
        due.set_date("2017-12-25");
        let mut task = Task::create("Test Task");
        task.set_due(Some(due));
        assert_eq!(task.due().unwrap().date(), Some(String::from("2017-12-25")));

        let json = serde_json::to_string(&task).unwrap();
        println!("{}", json);
        assert!(json.contains("\"due_date\":\"2017-12-25\""));
    }

    #[test]
    fn create_and_serialize_task() {
        let new_task = Task::create("Test Task");
        let json = serde_json::to_string(&new_task).unwrap();
        println!("{}", json);
        assert!(json.contains("\"content\":\"Test Task\""));
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

    #[test]
    fn update_task_properties() {
        let mut task = Task::create("Test Task");
        task.set_content("New Task Name");
        task.set_priority(3);
        task.set_completed(true);
        task.add_label_id(10);
        task.add_label_id(4);
        task.add_label_id(1);
        assert_eq!(task.label_ids(), [10, 4, 1]);
        task.remove_label_id(4);
        assert_eq!(task.label_ids(), [10, 1]);

        println!("{:?}", task);

        let json = serde_json::to_string(&task).unwrap();
        println!("{}", json);
        assert!(json.contains("New Task Name"));
    }
}

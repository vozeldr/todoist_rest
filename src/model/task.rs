use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Deserialize, Debug)]
pub struct Due {
    /// Human defined date in arbitrary format
    string: String,
    /// Date in format YYYY-MM-DD corrected to user’s timezone
    date: String,
    /// Only returned if exact due time set (i.e. it’s not a whole-day task), date and time in RFC3339 format in UTC;
    datetime: Option<String>,
    /// Only returned if exact due time set, user’s timezone definition either in tzdata-compatible format (“Europe/Berlin”) or as a string specifying east of UTC offset as “UTC±HH:MM” (i.e. “UTC-01:00”);
    timezone: Option<String>
}

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

    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn project_id(&self) -> &Option<u32> {
        &self.project_id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn completed(&self) -> &bool {
        &self.completed
    }

    pub fn label_ids(&self) -> &Vec<u32>{
        &self.label_ids
    }

    pub fn order(&self) -> &Option<u32>{
        &self.order
    }

    pub fn indent(&self) -> &Option<u32>{
        &self.indent
    }

    pub fn priority(&self) -> u32{
        self.priority
    }

    pub fn due(&self) -> &Option<Due> {
        &self.due
    }

    pub fn url(&self) -> &Option<String> {
        &self.url
    }

    pub fn comment_count(&self) -> &Option<u32> {
        &self.comment_count
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut state = serializer.serialize_struct("Task", 9)?;

        state.serialize_field("content", &self.content)?;
        state.serialize_field("project_id", &self.project_id)?;
        state.serialize_field("order", &self.order)?;
        state.serialize_field("label_ids", &self.label_ids)?;
        state.serialize_field("priority", &self.priority)?;

        match &self.due {
            &Some(ref due) => {
                state.serialize_field("due_string", &due.string)?;
                state.serialize_field("due_date", &due.date)?;
                state.serialize_field("due_datetime", &due.datetime)?;
                state.serialize_field("due_lang", "en")?;
            },
            &None => {
                state.serialize_field("due_string", "")?;
                state.serialize_field("due_date", "")?;
                state.serialize_field("due_datetime", "")?;
                state.serialize_field("due_lang", "")?;
            }
        };

        state.end()
    }
}
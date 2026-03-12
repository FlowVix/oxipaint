use indexmap::IndexMap;

pub struct ProjectInfo {}

pub struct ProjectState {
    pub projects: IndexMap<u32, ProjectInfo>,
    pub current_project: Option<u32>,
}

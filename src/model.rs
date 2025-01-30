#[derive(Debug)]
pub struct Model {
    pub code: String,
    pub name: String,
    pub description: String,
    pub model: String,
    pub tech: Tech,
    pub status: String,
}

#[derive(Debug)]
pub struct Tech {
    pub name: Vec<String>,
}
#[derive(Debug)]
struct Model {
    pub code: String,
    pub name: String,
    pub description: String,
    pub model: String,
    pub tech: Tech,
    pub status: String,
}


#[derive(Debug)]
struct Tech {
    pub name: Vec<String>,
}

fn main() {
    todo!("implement new model repository and service repository");
}

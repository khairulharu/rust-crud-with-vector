#[derive(Debug)]
pub struct Model <'a> {
     pub code: &'a str,
     pub name: &'a str,
     pub description: String,
     pub model: &'a str,
     pub tech: Tech,
     pub status: &'a str,
}

#[derive(Debug)]
pub struct Tech {
     pub name: Vec<String>,
}
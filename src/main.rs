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

struct ModelRepository {
    data: Vec<Model>,
}

impl ModelRepository {
    fn new() -> Self {
        ModelRepository {
            data: Vec::<Model>::new(),
        }
    }

    fn create(&mut self, request: Model) {
        self.data.push(request);
    }

    fn read_by_code(&mut self, code: String) -> Result<Model, String> {
        todo!("map the data self.data and serach data that same code if not found return String error")
    }
}

fn main() {
    todo!("implement new model repository and service repository");
}

#[test]
fn test_model_repository() {
    let mut model_repository = ModelRepository::new();

    model_repository.create(Model {
        code: "123".to_string(),
        name: "my_name".to_string(),
        description: "this is a expression of your imagination".to_string(),
        model: "text".to_string(),
        tech: Tech {
            name: vec!["alloc".to_string(), "this is data".to_string()],
        },
        status: "activated".to_string(),
    });

    assert!(!model_repository.data.is_empty());

    let result = model_repository.data.iter().find(|data| data.code == "1234".to_string());

    match result {
        Some(data) => println!("{:?}", data),
        None => println!("Not Found"),
    };
}

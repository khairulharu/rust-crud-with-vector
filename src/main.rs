mod model;
mod repository;
mod server;
mod status;

mod tests;

fn main() {
    todo!("implement new model repository and service repository");
}

#[test]
fn test_model_repository() {
    let mut model_repository = repository::ModelRepository::new();

    model_repository.create(model::Model {
        code: "123".to_string(),
        name: "my_name".to_string(),
        description: "this is result of your imagination".to_string(),
        model: "text".to_string(),
        tech: model::Tech {
            name: vec!["alloc".to_string(), "this is data".to_string()],
        },
        status: "activated".to_string(),
    });

    assert!(!model_repository.data.is_empty());

    let result = model_repository.read_by_code("123".to_string());
    match result {
        Ok(data) => println!("{:?}", data),
        Err(err) => println!("{}", err),
    };

    let all_models = model_repository.read_all();

    assert_eq!(all_models.len(), 1);
}
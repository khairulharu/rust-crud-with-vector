#[cfg(test)]

use super::*;

#[test]
fn test_model_repository() {
     let model_repository = repository::ModelRepository::new();

     let test_model = model::Model {
        code: "example code".to_string(),
        name: "example name".to_string(),
        description: "lorem ipsum dolor sit amet".to_string(),
        model: "tech".to_string(),
        tech: model::Tech {
          name: vec!["thing".to_string(), "thinging".to_string()],
        },
        status: todo!(),
    };

     model_repository.create();
}
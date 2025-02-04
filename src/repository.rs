use crate::model::Model;

#[derive(Debug)]
pub enum RepositoryError {
     NotFound(String),
     Other(String)
}

pub struct ModelRepository {
     pub data: Vec<Model>,
}

impl ModelRepository {
     pub fn new() -> Self {
          ModelRepository {
               data: Vec::<Model>::new(),
          }
     }
     pub fn read_all(&self) -> Vec<&Model> {
          self.data.iter().collect()
     }

     pub fn create(&mut self, request: Model) {
          self.data.push(request);
     }

     pub fn read_by_code(&self, code: &str) -> Result<&Model, RepositoryError> {
          self.data
              .iter()
              .find(|m| m.code == code)
              .ok_or(RepositoryError::NotFound("Model Not Found".to_string()))
     }

     pub fn update(&mut self, code: &str, new_model: Model) -> Result<String, RepositoryError> {
          let result = self.data.iter_mut().find(|m| m.code == code);

          match result {
               Some(recent_model) => {
                    *recent_model = new_model;
                    Ok("Update success".to_string())
               },
               None => Err(RepositoryError::NotFound("Model Not Found".to_string()))
          }
     }

     pub fn delete(&mut self, code: &str) -> Result<String, RepositoryError> {
          let initial_len = self.data.len();
          self.data.retain(|model| model.code != code);
          if self.data.len() < initial_len {
               Ok("Deleted Model".to_string())
          } else {
               Err(RepositoryError::NotFound("Model Not Found".to_string()))
          }
     }
}

#[test]
fn test_model_repository() {
     let mut model_repository: ModelRepository = ModelRepository::new();

     let model_test: Model = Model {
          code: String::from("huxUsndsHdhfd"),
          name: String::from("Car"),
          description: String::from("another car that stolen from neightberhod, that on repair, so the car should be cannt run"),
          model: String::from("axdv"),
          tech: crate::model::Tech {
               name: vec!["vw".to_string(), "bmw".to_string()],
          },
          status: String::from("activated"),
     };

     model_repository.create(model_test.clone());

     let all_models = model_repository.read_all();

     assert_eq!(all_models.len(), 1);

     let result= model_repository.read_by_code(model_test.clone().code.as_str()).unwrap();

     assert_eq!(result.code, model_test.code);

     model_repository.create(model_test.clone());

     let all_models = model_repository.read_all();
     assert_eq!(all_models.len(), 2);

     //as this later on
     model_repository.create(model_test.clone());

     assert_eq!(all_models.len(), 3);
}
use crate::model::Model;

pub struct ModelRepository {
     pub data: Vec<Model>,
}

impl ModelRepository {
     pub fn new() -> Self {
          ModelRepository {
               data: Vec::<Model>::new(),
          }
     }
     pub fn read_all(&mut self) -> Vec<&Model> {
          self.data.iter().map(|data| data).collect()
     }

     pub fn create(&mut self, request: Model) {

          self.data.push(request);
     }

     pub fn read_by_code(&mut self, code: String) -> Result<&Model, String> {
          let result = self.data.iter().find(|data| data.code == code);
          match result {
            Some(data) => Ok(data),
            None => Err("Error Model Not Found".to_string()),
          }
     }

     pub fn update(&mut self, code: String, model: Model) -> Result<String, String> {
          todo!("implement update model via code string");
     }

     pub fn delete(&mut self, code: String) -> Result<String, String> {
          self.data.
     }
}

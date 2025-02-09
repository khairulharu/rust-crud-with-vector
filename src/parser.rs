use crate::model::{Model, Tech};

fn parse_model_request(request: &str) -> Result<Model, String> {
     let mut model: std::mem::MaybeUninit<Model>;
     
     if request.contains("code:") {
     }

     return Ok(model);
}

#[test]
fn test_parse_model_request() {
     if let Ok(model) = parse_model_request("code:ALABAMA") {
          println!("{:?}", model);
     }
}
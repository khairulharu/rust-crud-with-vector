mod model;
mod repository;

fn main() {
    let mut main_data: Vec<model::Model> = Vec::new();

    main_data.push(model::Model{
        code: "10",
        name: "jalangkote",
        description: "halo dek kerja atau kuliah".to_string(),
        model: "food",
        tech: model::Tech {
            name: vec!["halo dek".to_string(), "halo banf".to_string()]
        },
        status: "active",
    });

    println!("{:?}", main_data);
}

mod model;

fn main() {
    let a = model::VertexTrait {
        latitude : 1 as f32,
        longitude: 0 as f32
    };
    println!("{}", a.latitude);
}

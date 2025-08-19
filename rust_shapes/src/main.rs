 // Crie um novo projeto em Rust

 // Crie um arquivo chamado "shapes.rs" e coloque-o na pasta do projeto

 // No arquivo "shapes.rs", crie uma trait chamada "Shape" com os métodos area, perimeter e draw

 // Crie uma struct que implemente a trait Shape, por exemplo "Circle"

 // Implemente os métodos area, perimeter e draw na struct "Circle"

 // No arquivo "main.rs", importe a trait e a struct "Circle" usando "use"

 // Crie uma instância da struct "Circle" e chame os métodos area, perimeter e draw nela

 // Teste o projeto executando o comando "cargo run" no terminal


mod shapes;
use shapes::{Circle, Shape};

fn main() {
    let c = Circle{diameter: 1.0};
    println!("circle area {} and perimeter {}", c.area(), c.perimeter());
    c.draw();
}

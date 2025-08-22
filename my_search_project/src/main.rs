use my_search_project::{run, Config};
use std::env;

fn main() {
    let args = env::args();
    let config = Config::build(args.collect::<Vec<String>>())
        .expect("Erro ao tratar os argumentos de linha de comando");
    println!("config = {:?}", config);
    run(config).expect("Erro ao executar a busca");
}

use my_search_project::{run, Config};

#[test]
fn test_sem_argumentos() {
    let config = Config::build(Vec::new());
    assert!(config.is_err());
}

#[test]
fn test_arquivo_nao_existe() {
    let config = Config::build(vec![
        "".to_string(),
        "a".to_string(),
        "jsflsjdf.txt".to_string(),
    ])
    .unwrap();
    assert!(run(config).is_err());
}

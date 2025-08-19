use rust_lifetime::maior_string;

#[test]
fn test_vetor_vazio() {
    let v = Vec::new();
    let maior = maior_string(&v);
    assert_eq!(maior, None);
}

#[test]
fn test_vetor_nao_vazio() {
    let v = vec!["", "0"];
    let maior = maior_string(&v);
    assert_eq!(maior, Some("0"));
}

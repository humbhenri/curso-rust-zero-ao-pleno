pub fn maior_string<'a>(strings: &Vec<&'a str>) -> Option<&'a str> {
    if strings.is_empty() {
        return None;
    }
    let mut maior = strings[0];
    for s in strings {
        if s.len() > maior.len() {
            maior = s;
        }
    }
    Some(maior)
}

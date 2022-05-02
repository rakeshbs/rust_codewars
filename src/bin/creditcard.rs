pub fn maskify(cc: &str) -> String {
    if cc.len() <= 4 { return String::from(cc) };
    let hash_len = cc.len() - 4;
    let hash_str = "#".repeat(hash_len);
    let formatted =  format!("{}{}",hash_str, &cc[hash_len..]);
    formatted
 }

pub fn main() {
    assert_eq!(maskify("4556364607935616"), "############5616");
    assert_eq!(maskify("1"), "1");
    assert_eq!(maskify("11111"), "#1111");
}

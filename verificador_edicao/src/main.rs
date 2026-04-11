fn esta_a_um_passo(str1: &str, str2: &str) -> bool {
    let total_str1 = str1.len() as i32;
    let mut total_str2 = 0;

    for i in str1.chars(){
        if str2.contains(i){
            total_str2 += 1
        }
    }

    total_str1 - total_str2 == 1
}
 
fn main() {
    // Testes de exemplo
    let str1 = "pale";
    let str2 = "ple";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str1, str2));
 
    let str3 = "pales";
    let str4 = "pale";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str3, str4));
 
    let str5 = "pale";
    let str6 = "bale";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str5, str6));
 
    let str7 = "pale";
    let str8 = "bibo";
    println!("Strings estão a uma edição de distância: {}", esta_a_um_passo(str7, str8));
}
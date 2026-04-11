fn main() {
    let original_str = "aabcccccaaa";
    let compressed_str = compress_string(original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);
 
    let other_str = "abcdefgh";
    let compressed_other = compress_string(other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}

fn compress_string(original_str: &str) -> String {
    let mut result: String = String::new();
    let mut count: i32 = 1;
    let chars: Vec<char> =  original_str.chars().collect();

    for i in 0..chars.len(){
        if i + 1 < chars.len() && chars[i] == chars[i + 1] {
            count += 1;
        } else {
            result.push(chars[i]);
            result.push_str(&count.to_string());
            count = 1;
        }
    }
    
    if result.len() >= original_str.len() {
        original_str.to_string()
    } else {
        result
    }
}
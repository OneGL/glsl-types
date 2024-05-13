use crate::uniforms::{Uniform, UniformType};

pub fn capitalize_first_letter(s: &str) -> String {
    s.chars().next().unwrap().to_uppercase().collect::<String>() + &s[1..]
}

pub fn extract_uniforms(file: &String) -> Vec<Uniform> {
    let mut uniforms = Vec::new();

    let file = remove_comments(file);
    let file = file.replace("\n", "");

    let words = file.split_whitespace();

    // Check each word for words with the ";" character and convert those words into multiple words
    let words: Vec<&str> = words
        .map(|word| word.split(";").collect::<Vec<&str>>())
        .flatten()
        .collect();

    let mut i = 0;
    while i < words.len() {
        if words[i] == "uniform" {
            let uniform_type = words[i + 1];
            let uniform_name = words[i + 2].split(";").collect::<Vec<&str>>()[0];

            let uniform_type = match UniformType::from_str(uniform_type) {
                Ok(uniform_type) => uniform_type,
                Err(_) => {
                    println!(
                        "Unknown uniform type: {}. Skipping types for uniform: {}",
                        uniform_type, uniform_name
                    );

                    i += 1;
                    continue;
                }
            };

            let uniform = Uniform {
                name: uniform_name.to_string(),
                uniform_type: uniform_type,
            };

            uniforms.push(uniform);
        }
        i += 1;
    }

    return uniforms;
}

fn remove_comments(file: &String) -> String {
    let mut result = String::new();
    let mut chars = file.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '/' {
            if let Some(&'/') = chars.peek() {
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    }
                }
            } else if let Some(&'*') = chars.peek() {
                while let Some(c) = chars.next() {
                    if c == '*' {
                        if let Some(&'/') = chars.peek() {
                            chars.next();
                            break;
                        }
                    }
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    return result;
}

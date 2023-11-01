use std::fs::read_to_string;

pub fn split_file(filename: &str) -> Vec<Vec<String>> {
    let mut paragraphs = Vec::new();
    let mut paragraph = Vec::new();

    let content = read_to_string(filename).expect("could not read file");

    for mut line in content.lines() {
        line = line.trim();

        if line.is_empty() {
            paragraphs.push(paragraph.clone());
            paragraph.clear();
        } else {
            paragraph.push(line.into());
        }
    }

    if paragraph.len() > 0 {
        paragraphs.push(paragraph.clone());
    }

    paragraphs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_file() {
        let text = "./src/test-output/test-split.txt";

        let paragraphs = split_file(text);

        assert_eq!(
            vec![
                vec!["test".to_string()],
                vec!["test".to_string()],
                vec!["test".to_string()],
                vec!["test".to_string()]
            ],
            paragraphs
        );
    }
}

fn main() {
    let ans = pig_latin("first apple");
    println!("{}", ans);
}
fn pig_latin(s: &str) -> String {
    let mut ans_str = String::new();
    for word in s.split_whitespace() {
        let mut first_c = ' ';
        let mut pig_word = String::new();
        let mut vowel_flag = false;
        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                match c {
                    'a' | 'i' | 'u' | 'e' | 'o' => {
                        vowel_flag = true;
                        break;
                    }
                    _ => {}
                }
                first_c = c;
            } else {
                pig_word.push(c);
            }
        }
        if vowel_flag {
            pig_word.push_str(word);
            pig_word.push_str("hay");
        } else {
            pig_word.push(first_c);
            pig_word.push_str("ay");
        }
        ans_str.push_str(&pig_word);
        ans_str.push_str(" ");
    }
    ans_str
}

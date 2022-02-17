fn main() {
    let mut s = "我爱你！中国".to_string();
    let r = s.as_mut();
    if let Some((s1, s2)) = split(r, '！') {
        println!("s1: {:?}, s2: {:?}", s1, s2);
    }
}

fn split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);
    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();
        println!("{}", pos);
        unsafe {
            (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..len))
        }
    })
}
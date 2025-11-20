pub trait IntoBaseExt {
    fn into_base(&self) -> String;
    fn escape(&self, dup_quote: bool) -> String;
}

impl IntoBaseExt for str {
    fn into_base(&self) -> String {
        let mut s = self;
        if s.starts_with("$") {
            s = &s[1..];
        }
        s = &s[1..(s.len() - 1)];
        s.to_string()
    }

    fn escape(&self, dup_quote: bool) -> String {
        let mut ret = self
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\'", "\\\'")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t");
        if dup_quote {
            ret = ret.replace("{", "{{").replace("}", "}}");
        }
        ret
    }
}

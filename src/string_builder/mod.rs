use crate::arraylist::ArrayList;

pub struct StringBuilder {
    // TODO make this an array and control the resizing ourselves
    content: ArrayList<String>,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder {
            content: ArrayList::new(),
        }
    }

    pub fn append<S: Into<String>>(mut self, s: S) -> Self {
        self.content.push(s.into());
        self
    }

    pub fn capacity(&self) -> usize {
        self.content.cap()
    }

    pub fn length(&self) -> u64 {
        self.content.iter().map(|s| s.chars().count() as u64).sum()
    }
}

impl From<StringBuilder> for String {
    fn from(sb: StringBuilder) -> Self {
        // TODO is this effecient? Or should we allocate our own string, calculating the size ahead of time.
        sb.content.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::string_builder::StringBuilder;

    #[test]
    fn empty() {
        let sb = StringBuilder::new();

        let actual: String = sb.into();
        assert_eq!("", actual);
    }

    #[test]
    fn append() {
        let sb = StringBuilder::new()
            .append("apples")
            .append(String::from(" "))
            .append("bananas")
            .append(String::from("oranges"))
            .append("");

        assert_eq!(21, sb.length());
        let actual: String = sb.into();
        assert_eq!("apples bananasoranges", actual);
    }
}

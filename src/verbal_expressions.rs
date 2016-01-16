#![crate_name = "verbal_expressions"]
#![crate_type = "lib"]
//#![desc = "Verbal Expressions implementation for Rust"]
//#![license = "MIT"]

extern crate regex;
use regex::Regex;

pub struct Verex {
  prefix: String,
  source: String,
  suffix: String,
}

impl Verex {
  fn from(prefix: String, source: String, suffix: String) -> Verex {
    Verex { prefix: prefix, source: source, suffix: suffix }
  }

  pub fn new() -> Verex {
    Verex::from(String::new(), String::new(), String::new())
  }

  pub fn start_of_line(mut self) -> Self {
    self.prefix.push('^');
    self
  }

  pub fn end_of_line(mut self) -> Self {
    self.suffix.push('$');
    self
  }

  fn add(mut self, value: &str) -> Self {
    self.source.push_str(value);
    self
  }

  pub fn find(self, value: &str) -> Self {
    self.add(&format!("(?:{})", value))
  }

  pub fn then(self, value: &str) -> Self {
    self.find(value)
  }

  pub fn maybe(self, value: &str) -> Self {
    self.add(&format!("(?:{})?", value))
  }

  pub fn anything(self) -> Self {
    self.add("(?:.*)")
  }

  pub fn anything_but_not(self, value: &str) -> Verex {
    self.add(&format!("(?:[^{}]*)", value))
  }

  pub fn something(self) -> Verex {
    self.add("(?:.+)")
  }

  pub fn something_but_not(self, value: &str) -> Verex {
    self.add(&format!("(?:[^{}]+)", value))
  }

  pub fn line_break(self) -> Verex {
    self.add("(?:(?:\\n)|(?:\\r\\n))")
  }

  pub fn br(self) -> Verex {
    self.line_break()
  }

  pub fn tab(self) -> Verex {
    self.add("(?:\\t)")
  }

  pub fn word(self) -> Verex {
    self.add("(?:\\w+)")
  }

  pub fn non_word(self) -> Verex {
    self.add("(?:\\W+)")
  }

  pub fn digit(self) -> Verex {
    self.add("(?:\\d)")
  }

  pub fn non_digit(self) -> Verex {
    self.add("(?:\\D)")
  }

  pub fn space(self) -> Verex {
    self.add("(?:\\s)")
  }

  pub fn non_space(self) -> Verex {
    self.add("(?:\\S)")
  }

  pub fn any_of(self, value: &str) -> Verex {
    self.add(&format!("[{}]", value))
  }

  pub fn any(self, value: &str) -> Verex {
    self.any_of(value)
  }

  pub fn range(self, pairs: &[(&str, &str)]) -> Verex {
    let ranges = pairs.iter().fold(String::new(), |buf, &(from, to)| {
      buf + &format!("{}-{}", from, to)
    });

    self.add(&format!("[{}]", ranges))
  }

  pub fn add_modifier(self, modifier: &str) -> Verex {
    self.add(&format!("(?{})", modifier))
  }

  pub fn remove_modifier(self, modifier: &str) -> Verex {
    self.add(&format!("(?-{})", modifier))
  }

  pub fn with_any_case(self) -> Verex {
    self.add_modifier("i")
  }

  pub fn with_specific_case(self) -> Verex {
    self.remove_modifier("i")
  }

  pub fn search_multiline(self) -> Verex {
    self.add_modifier("m")
  }

  pub fn search_oneline(self) -> Verex {
    self.remove_modifier("m")
  }

  pub fn multiple(self, value: &str, counts: &[i32]) -> Verex {
    match counts.len() {
      0 => self.then(value).one_or_more(),
      1 => self.then(value).count(counts[0]),
      _ => self.then(value).count_range(counts[0], counts[1]),
    }
  }

  pub fn one_or_more(self) -> Verex {
    self.add("+")
  }

  pub fn zero_or_more(self) -> Verex {
    self.add("*")
  }

  pub fn count(self, count: i32) -> Verex {
    self.add(&format!("{{{}}}", count))
  }

  pub fn count_range(self, from: i32, to: i32) -> Verex {
    self.add(&format!("{{{},{}}}", from, to))
  }

  pub fn at_least(self, from: i32) -> Verex {
    self.add(&format!("{{{},}}", from))
  }

  pub fn or(mut self, value: &str) -> Self {
    let p = if self.prefix.contains("(") { "" } else { "(" };
    let s = if self.suffix.contains(")") { "" } else { ")" };
    self.prefix.push_str(p);
    self.suffix.push_str(s);
    self.add(")|(").then(value)
  }

  pub fn begin_capture(mut self) -> Self {
    self.suffix.push(')');
    self.add("(")
  }

  pub fn end_capture(self) -> Verex {
    let suffix = self.suffix[0..self.suffix.len()-1].to_string();
    Verex::from(self.prefix, self.source, suffix).add(")")
  }

  pub fn is_match(self, text: &str) -> bool {
    self.as_regex().is_match(text)
  }

  pub fn captures(self, text: &str) -> Vec<String> {
    match self.as_regex().captures(text) {
      Some(captures) => captures.iter().map(|x| x.expect("captures method failed").to_string()).collect(),
      None => Vec::new(),
    }
  }

  pub fn split(self, text: &str) -> Vec<String> {
    self.as_regex().split(text).map(|x| x.to_string()).collect()
  }

  pub fn replace(self, text: &str, rep: &str) -> String {
    self.as_regex().replace(text, rep)
  }

  pub fn as_string(self) -> String {
    self.prefix + &self.source + &self.suffix
  }

  pub fn as_regex(self) -> Regex {
    Regex::new(&self.as_string()).unwrap()
  }
}

#[cfg(test)]
mod test {
  use super::Verex;

  #[test]
  fn test_start_of_line() {
    assert_eq!(&Verex::new().start_of_line().as_string(), "^");
  }

  #[test]
  fn test_end_of_line() {
    assert_eq!(&Verex::new().end_of_line().as_string(), "$");
  }

  #[test]
  fn test_add() {
    assert_eq!(&Verex::new().add("Karen").as_string(), "Karen");
    assert_eq!(&Verex::new().add("Karen").add("Karen").as_string(), "KarenKaren");
  }
    
  #[test]
  fn test_find() {
    assert!(Verex::new().find("Karen").is_match("Karen"));
    assert!(!Verex::new().find("Karen").is_match("Alice"));
  }

  #[test]
  fn test_then() {
    assert!(Verex::new().then("Karen").is_match("Karen"));
    assert!(!Verex::new().then("Karen").is_match("Alice"));
  }

  #[test]
  fn test_maybe() {
    assert!(Verex::new().maybe("Karen").is_match("Karen"));
    assert!(Verex::new().maybe("Karen").is_match("Alice"));
  }

  #[test]
  fn test_anything() {
    assert!(Verex::new().anything().is_match(""));
    assert!(Verex::new().anything().is_match("Karen"));
  }

  #[test]
  fn test_anything_but_not() {
    assert!(!Verex::new().start_of_line().anything_but_not("r").end_of_line().is_match("Karen"));
    assert!(Verex::new().start_of_line().anything_but_not("r").end_of_line().is_match("Alice"));
  }

  #[test]
  fn test_something() {
    assert!(!Verex::new().something().is_match(""));
    assert!(Verex::new().something().is_match("Karen"));
  }

  #[test]
  fn test_something_but_not() {
    assert!(!Verex::new().start_of_line().something_but_not("r").end_of_line().is_match("Karen"));
    assert!(Verex::new().start_of_line().something_but_not("r").end_of_line().is_match("Alice"));
  }

  #[test]
  fn test_line_break() {
    assert!(Verex::new().line_break().is_match("\n"));
    assert!(Verex::new().line_break().is_match("\n\r"));
  }

  #[test]
  fn test_br() {
    assert!(Verex::new().br().is_match("\n"));
    assert!(Verex::new().br().is_match("\n\r"));
  }

  #[test]
  fn test_tab() {
    assert!(Verex::new().tab().is_match("\t"));
  }

  #[test]
  fn test_word() {
    assert!(Verex::new().word().is_match("Karen"));
    assert!(!Verex::new().word().is_match("*"));
  }

  #[test]
  fn test_non_word() {
    assert!(!Verex::new().non_word().is_match("Karen"));
    assert!(Verex::new().non_word().is_match("*"));
  }

  #[test]
  fn test_digit() {
    assert!(Verex::new().digit().is_match("42"));
    assert!(!Verex::new().digit().is_match("Karen"));
  }

  #[test]
  fn test_non_digit() {
    assert!(!Verex::new().non_digit().is_match("42"));
    assert!(Verex::new().non_digit().is_match("Karen"));
  }

  #[test]
  fn test_space() {
    assert!(Verex::new().space().is_match(" "));
    assert!(Verex::new().space().is_match("　"));
    assert!(!Verex::new().space().is_match("Karen"));
  }

  #[test]
  fn test_non_space() {
    assert!(!Verex::new().non_space().is_match(" "));
    assert!(!Verex::new().non_space().is_match("　"));
    assert!(Verex::new().non_space().is_match("Karen"));
  }

  #[test]
  fn test_any_of() {
    assert!(Verex::new().any_of("Karen").is_match("K"));
    assert!(!Verex::new().any_of("Karen").is_match("*"));
  }

  #[test]
  fn test_any() {
    assert!(Verex::new().any("Karen").is_match("K"));
    assert!(!Verex::new().any("Karen").is_match("*"));
  }

  #[test]
  fn test_range() {
    assert!(Verex::new().range(&[("a", "z")]).is_match("x"));
    assert!(!Verex::new().range(&[("a", "z")]).is_match("*"));
    assert!(Verex::new().range(&[("a", "z"), ("A", "Z")]).is_match("X"));
    assert!(!Verex::new().range(&[("a", "z"), ("A", "Z")]).is_match("*"));
  }

  #[test]
  fn test_add_modifier() {
    assert_eq!(&Verex::new().add_modifier("x").as_string(), "(?x)");
  }

  #[test]
  fn test_remove_modifier() {
    assert_eq!(&Verex::new().remove_modifier("x").as_string(), "(?-x)");
  }

  #[test]
  fn test_with_any_case() {
    assert!(Verex::new().with_any_case().then("Karen").is_match("karen"));
  }

  #[test]
  fn test_with_specific_case() {
    assert!(!Verex::new().with_specific_case().then("Karen").is_match("karen"));
  }

  #[test]
  fn test_search_multiline() {}

  #[test]
  fn test_search_oneline() {}

  #[test]
  fn test_multiple() {
    assert_eq!(&Verex::new().multiple("x", &[]).as_string(), "(?:x)+");
    assert_eq!(&Verex::new().multiple("x", &[1]).as_string(), "(?:x){1}");
    assert_eq!(&Verex::new().multiple("x", &[1, 2]).as_string(), "(?:x){1,2}");
  }

  #[test]
  fn test_one_or_more() {
    assert!(!Verex::new().then("x").one_or_more().is_match(""));
    assert!(Verex::new().then("x").one_or_more().is_match("x"));
  }

  #[test]
  fn test_zero_or_more() {
    assert!(Verex::new().then("x").zero_or_more().is_match(""));
    assert!(Verex::new().then("x").zero_or_more().is_match("x"));
  }

  #[test]
  fn test_count() {
    assert!(!Verex::new().then("x").count(1).is_match(""));
    assert!(Verex::new().then("x").count(1).is_match("x"));
  }

  #[test]
  fn test_count_range() {
    assert!(!Verex::new().then("x").count_range(1, 2).is_match(""));
    assert!(Verex::new().then("x").count_range(1, 2).is_match("x"));
  }

  #[test]
  fn test_at_least() {
    assert!(!Verex::new().then("x").at_least(2).is_match("x"));
    assert!(Verex::new().then("x").at_least(2).is_match("xx"));
  }

  #[test]
  fn test_or() {
    assert!(Verex::new().then("Karen").or("Alice").is_match("Karen"));
    assert!(Verex::new().then("Karen").or("Alice").is_match("Alice"));
  }

  #[test]
  fn test_begin_capture() {
    let v = Verex::new().begin_capture().then("K");
    assert_eq!(&v.captures("Karen")[1], "K");
  }

  #[test]
  fn test_end_capture() {
    let v = Verex::new().begin_capture().then("K").end_capture().then("aren");
    assert_eq!(&v.captures("Karen")[1], "K");
  }
}

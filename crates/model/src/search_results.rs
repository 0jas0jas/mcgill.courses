use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResults {
  pub courses: Vec<Course>,
  pub instructors: Vec<Instructor>,
}

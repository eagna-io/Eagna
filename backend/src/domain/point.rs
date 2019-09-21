#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Serialize, Deserialize)]
pub struct Point(u32);

impl Point {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

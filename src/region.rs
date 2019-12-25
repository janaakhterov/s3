#[derive(Debug, Copy, Clone)]
pub enum Region {
    UsEast1,
}

impl Into<String> for Region {
    fn into(self) -> String {
        match self {
            Region::UsEast1 => "us-east-1",
        }
        .to_owned()
    }
}

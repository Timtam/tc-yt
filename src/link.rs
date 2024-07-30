use uuid::Uuid;

#[derive(Clone, Default, Eq, PartialEq)]
pub enum LinkType {
    #[default]
    None,
    Account(String),
}

#[derive(Clone)]
pub struct Link {
    pub id: Uuid,
    pub r#type: LinkType,
    pub name: String,
}

impl Link {
    pub fn new<I>(name: I, r#type: LinkType) -> Self
    where
        I: Into<String>,
    {
        Self {
            name: name.into(),
            r#type,
            id: Uuid::new_v4(),
        }
    }
}

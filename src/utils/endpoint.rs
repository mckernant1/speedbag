use crate::args::args::Endpoint;

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Vec<Endpoint> {
    fn to_string(&self) -> String {
        self.iter()
            .map(|it| format!("{:?}", it))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

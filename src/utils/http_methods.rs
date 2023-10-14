#[derive(Debug, PartialEq)]
enum HttpMethods {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Patch,
}

impl HttpMethods {
    fn get(&self) -> &str {
        match self {
            HttpMethods::Get => "GET",
            HttpMethods::Post => "POST",
            HttpMethods::Put => "PUT",
            HttpMethods::Delete => "DELETE",
            HttpMethods::Options => "OPTIONS",
            HttpMethods::Patch => "PATCH",
        }
    }
}

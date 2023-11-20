pub trait Req<State> {
    fn as_url(&self) -> &str;

    fn into_string(self) -> String;

    fn into_bytes(self) -> Vec<u8>;
}

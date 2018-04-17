use std::result::Result as Res;

pub struct Message {
    message: String,
    type_str: String
}

pub enum Error {
    ErrorMessage(Message)
}

type Response<T> = Res<T, Error>;
type NoContent = Res<(), Error>;

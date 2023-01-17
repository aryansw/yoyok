use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {}

pub type Parse<T> = Result<T, Error>;

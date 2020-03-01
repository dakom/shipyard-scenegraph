#[derive(thiserror::Error, Debug)]
pub enum MatrixError {
    #[error("cannot invert the matrix")]
    Invert
}
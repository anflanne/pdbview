use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliArgumentError {
    #[error("the value `{1}` is not valid for the parameter `{0}`")]
    InvalidValue(&'static str, String),
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("the PDB parsing library encountered an error: {0}")]
    PdbCrateError(#[from] pdb::Error),
}

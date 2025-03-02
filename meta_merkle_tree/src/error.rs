use thiserror::Error;

#[derive(Error, Debug)]
pub enum MerkleTreeError {
    #[error("Merkle Tree Validation Error: {0}")]
    MerkleValidationError(String),
    #[error("Merkle Root Error")]
    MerkleRootError,
    #[error("io Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Arithmetic Overflow/Underflow")]
    ArithmeticOverflow,
}

#[derive(Error, Debug)]
pub enum MerkleRootGeneratorError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

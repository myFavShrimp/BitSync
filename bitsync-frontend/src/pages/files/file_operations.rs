pub enum UserFileUploadDisplayError {
    InvalidPath,
    NoFileProvided,
    NoFileNameProvided,
    InternalServerError,
}

impl UserFileUploadDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "The path is invalid",
            Self::NoFileProvided => "No file was provided",
            Self::NoFileNameProvided => "No file name was provided",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum UserFileDownloadDisplayError {
    InvalidPath,
    InternalServerError,
}

impl UserFileDownloadDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "The path is invalid",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum UserFileDeletionDisplayError {
    InvalidPath,
    InternalServerError,
}

impl UserFileDeletionDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "The path is invalid",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum UserFileMoveDisplayError {
    InvalidPath,
    InternalServerError,
}

impl UserFileMoveDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "The path is invalid",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

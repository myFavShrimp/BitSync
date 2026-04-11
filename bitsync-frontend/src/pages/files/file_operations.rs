pub enum UserFileUploadDisplayError {
    InvalidPath,
    NoFileProvided,
    NoFileNameProvided,
    InternalServerError,
}

impl UserFileUploadDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "Path must not contain '..' segments",
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
            Self::InvalidPath => "Path must not contain '..' segments",
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
            Self::InvalidPath => "Path must not contain '..' segments",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

pub enum UserFileMoveDisplayError {
    InvalidPath,
    DestinationSameAsSource,
    InternalServerError,
}

impl UserFileMoveDisplayError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidPath => "Path must not contain '..' segments",
            Self::DestinationSameAsSource => "New path is the same as the old path",
            Self::InternalServerError => "An internal server error occurred",
        }
    }
}

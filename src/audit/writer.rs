use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use super::AuditRecord;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AuditLogPath(pub PathBuf);

impl AuditLogPath {
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AuditWriteResult {
    pub path: AuditLogPath,
}

#[derive(Debug)]
pub enum AuditWriteError {
    Open {
        path: AuditLogPath,
        source: io::Error,
    },
    Serialize {
        path: AuditLogPath,
        source: serde_json::Error,
    },
    Write {
        path: AuditLogPath,
        source: io::Error,
    },
    Flush {
        path: AuditLogPath,
        source: io::Error,
    },
}

impl std::fmt::Display for AuditWriteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open { path, source } => {
                write!(
                    formatter,
                    "failed to open audit log {}: {source}",
                    path.as_path().display()
                )
            }
            Self::Serialize { path, source } => {
                write!(
                    formatter,
                    "failed to serialize audit record for {}: {source}",
                    path.as_path().display()
                )
            }
            Self::Write { path, source } => {
                write!(
                    formatter,
                    "failed to append audit log {}: {source}",
                    path.as_path().display()
                )
            }
            Self::Flush { path, source } => {
                write!(
                    formatter,
                    "failed to flush audit log {}: {source}",
                    path.as_path().display()
                )
            }
        }
    }
}

impl std::error::Error for AuditWriteError {}

pub trait AuditSink {
    fn append(&self, record: &AuditRecord) -> Result<AuditWriteResult, AuditWriteError>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AuditWriter {
    path: AuditLogPath,
}

impl AuditWriter {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: AuditLogPath(path.into()),
        }
    }

    fn open_append_file(&self) -> Result<File, AuditWriteError> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path.as_path())
            .map_err(|source| AuditWriteError::Open {
                path: self.path.clone(),
                source,
            })
    }
}

impl AuditSink for AuditWriter {
    fn append(&self, record: &AuditRecord) -> Result<AuditWriteResult, AuditWriteError> {
        let mut file = self.open_append_file()?;
        let line = serde_json::to_string(record).map_err(|source| AuditWriteError::Serialize {
            path: self.path.clone(),
            source,
        })?;

        file.write_all(line.as_bytes())
            .and_then(|_| file.write_all(b"\n"))
            .map_err(|source| AuditWriteError::Write {
                path: self.path.clone(),
                source,
            })?;
        file.flush().map_err(|source| AuditWriteError::Flush {
            path: self.path.clone(),
            source,
        })?;

        Ok(AuditWriteResult {
            path: self.path.clone(),
        })
    }
}

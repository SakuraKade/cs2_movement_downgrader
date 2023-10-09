use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub(crate) struct AutoExec {
    path: String,
    preset: String
}

impl AutoExec {
    pub fn new(config_path: String) -> Self {
        Self {
            path: config_path,
            preset: include_str!("autoexec.cfg").to_string()
        }
    }

    pub fn is_duplicate(&self) -> Result<bool, AutoExecError> {
        if !Path::new(&self.path).exists() {
            return Ok(false);
        }

        let content = fs::read_to_string(&self.path)?;
        if content.trim().contains(self.preset.trim()) {
            return Ok(true)
        }

        Ok(false)
    }

    pub fn append(&self) -> Result<(), AutoExecError> {
        if !Path::new(&self.path).exists() {
            File::create(&self.path)?;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{}", &self.preset)?;
        Ok(())
    }
}

//
// AutoexecError
//

pub enum AutoExecError {
    IoError(std::io::Error)

}

impl AutoExecError {
    fn message(&self) -> String {
        match self {
            AutoExecError::IoError(err) => err.to_string()
        }
    }
}

impl From<std::io::Error> for AutoExecError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Debug for AutoExecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for AutoExecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for AutoExecError {

}
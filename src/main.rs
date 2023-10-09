use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::panic::panic_any;
use crate::auto_exec::{AutoExec, AutoExecError};
use crate::cs2_locator::LocateError;

mod cs2_locator;
mod auto_exec;

fn main() {
    match alt_main() {
        Ok(_) => {}
        Err(err) => {
            on_error(Box::new(err))
        }
    }
    done()
}

fn done() {
    println!("Press enter to close . . .");
    let mut buff = String::new();
    let _ = std::io::stdin().read_line(&mut buff);
}

fn alt_main() -> Result<(), MainError> {
    let location = cs2_locator::Cs2Locator::locate()?;
    let auto_exec_path = cs2_locator::Cs2Locator::get_autoexec_path(location);

    let auto_exec = AutoExec::new(auto_exec_path);

    if auto_exec.is_duplicate()? {
        println!("Duplicate");
        return Ok(());
    }

    auto_exec.append()?;
    println!("Done!");
    Ok(())
}

fn on_error(error: Box<dyn Error>) {
    println!("{}", error.to_string());
    done();
    panic_any(error.to_string())
}

//
// MainError
//

enum MainError {
    LocateError(LocateError),
    AutoExecError(AutoExecError)
}

impl From<LocateError> for MainError {
    fn from(value: LocateError) -> Self {
        MainError::LocateError(value)
    }
}

impl From<AutoExecError> for MainError {
    fn from(value: AutoExecError) -> Self {
        Self::AutoExecError(value)
    }
}

impl MainError {
    fn message(&self) -> String {
        match self {
            MainError::LocateError(err) => err.to_string(),
            MainError::AutoExecError(err) => err.to_string()
        }
    }
}

impl Debug for MainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for MainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for MainError {

}
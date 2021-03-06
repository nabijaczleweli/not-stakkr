use std::io::Write;


/// Enum representing all possible values the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Outcome {
    /// No errors occured, everything executed correctly.
    NoError,
    /// The specified file would need to be overriden but was not allowed to.
    OverrideNoForce(String),
    /// The specified subsystem needs to be run beforehand to produce the specified file.
    RequiredFileFromSubsystemNonexistant {
        /// The subsystem that needs to be run.
        subsys: &'static str,
        /// The file the specified subsystem produces.
        fname: String,
    },
    /// The specified subsystem needs to be run beforehand to produce the specified data.
    RequiredDataFromSubsystemNonexistant {
        /// The subsystem that needs to be run.
        subsys: &'static str,
        /// The description of what needs to be produced.
        desc: String,
    },
    /// The Twitter API returned an error.
    TwitterAPIError(String),
    /// Failed to parse the specified file because of the specified errors.
    FileParsingFailed {
        /// The file that failed to parse.
        desc: &'static str,
        /// The parsing errors that occured.
        errors: Vec<String>,
    },
}

impl Outcome {
    /// Get the executable exit value from an `Outcome` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tweetr::Outcome;
    /// # use std::iter::FromIterator;
    /// let mut out = Vec::new();
    /// Outcome::OverrideNoForce("doctest".to_string()).print_error(&mut out);
    /// assert_eq!(String::from_iter(out.iter().map(|&i| i as char)),
    ///            "File \"doctest\" was not overriden to prevent data loss.\n\
    ///             Pass --force to override it.\n".to_string());
    /// ```
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Outcome::NoError => (),
            Outcome::OverrideNoForce(ref fname) => {
                writeln!(err_out, "File \"{}\" was not overriden to prevent data loss.", fname).unwrap();
                writeln!(err_out, "Pass --force to override it.").unwrap();
            }
            Outcome::RequiredFileFromSubsystemNonexistant { ref subsys, ref fname } => {
                writeln!(err_out, "Run the {} subsystem first to produce \"{}\".", subsys, fname).unwrap()
            }
            Outcome::RequiredDataFromSubsystemNonexistant { ref subsys, ref desc } => {
                writeln!(err_out, "Run the {} subsystem first to {}.", subsys, desc).unwrap()
            }
            Outcome::TwitterAPIError(ref error) => writeln!(err_out, "Twitter API error: {}", error).unwrap(),
            Outcome::FileParsingFailed { ref desc, ref errors } => {
                writeln!(err_out, "Failed to parse {}{}", desc, if errors.is_empty() { '.' } else { ':' }).unwrap();
                for err in errors {
                    writeln!(err_out, "  {}", err).unwrap()
                }
            }
        }
    }

    /// Get the executable exit value from an `Outcome` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::process::exit;
    /// # use tweetr::Outcome;
    /// exit(Outcome::NoError.exit_value());
    /// ```
    pub fn exit_value(&self) -> i32 {
        match *self {
            Outcome::NoError => 0,
            Outcome::OverrideNoForce(_) => 1,
            Outcome::RequiredFileFromSubsystemNonexistant { .. } |
            Outcome::RequiredDataFromSubsystemNonexistant { .. } => 2,
            Outcome::TwitterAPIError(_) => 3,
            Outcome::FileParsingFailed { .. } => 4,
        }
    }
}

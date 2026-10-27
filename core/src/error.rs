use std::fmt;

/// Errors reported by the uncertainty engine.
#[derive(Clone, Debug, PartialEq)]
pub enum UncError {
    /// The measurement-model expression could not be parsed.
    Parse {
        message: String,
        position: usize,
    },
    /// The expression refers to a name that is neither an input quantity nor
    /// an intermediate quantity.
    UnknownSymbol(String),
    /// A quantity name was defined twice.
    DuplicateName(String),
    InvalidDistribution(String),
    InvalidCorrelation(String),
    InvalidOption(String),
    /// The model is not defined (non-finite value) at the input estimates.
    ModelNotDefined(String),
    /// Monte Carlo draws produced non-finite model values.
    NonFiniteModelValues {
        count: usize,
        trials: usize,
    },
    /// The requested combination of methods is outside the domain of validity
    /// of the documented procedures.
    Unsupported(String),
}

impl fmt::Display for UncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UncError::Parse { message, position } => {
                write!(f, "parse error at character {position}: {message}")
            }
            UncError::UnknownSymbol(s) => write!(f, "unknown quantity '{s}'"),
            UncError::DuplicateName(s) => write!(f, "quantity '{s}' is defined more than once"),
            UncError::InvalidDistribution(s) => write!(f, "invalid distribution: {s}"),
            UncError::InvalidCorrelation(s) => write!(f, "invalid correlation: {s}"),
            UncError::InvalidOption(s) => write!(f, "invalid option: {s}"),
            UncError::ModelNotDefined(s) => write!(f, "model not defined: {s}"),
            UncError::NonFiniteModelValues { count, trials } => write!(
                f,
                "{count} of {trials} Monte Carlo trials gave a non-finite model value; \
                 the model is not defined over the whole support of the input distributions"
            ),
            UncError::Unsupported(s) => write!(f, "unsupported: {s}"),
        }
    }
}

impl std::error::Error for UncError {}

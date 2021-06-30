use std::{error, fmt};

#[derive(Debug)]
pub enum DbErr {
    Connection,
    Execution,
    Query,
    #[cfg(feature = "sqlx-dep")]
    Sqlx(sqlx::Error),
}

impl fmt::Display for DbErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Connection => write!(f, "{:?}", "Connection Error"),
            Self::Execution => write!(f, "{:?}", "Execution Error"),
            Self::Query => write!(f, "{:?}", "Query Error"),
            #[cfg(feature = "sqlx-dep")]
            Self::Sqlx(e) => write!(f, "{:?}", e),
        }
    }
}

impl error::Error for DbErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Connection => None,
            Self::Execution => None,
            Self::Query => None,
            #[cfg(feature = "sqlx-dep")]
            Self::Sqlx(e) => Some(e),
        }
    }
}

#[cfg(feature = "sqlx-dep")]
impl From<sqlx::Error> for DbErr {
    fn from(sqlx_err: sqlx::Error) -> Self {
        Self::Sqlx(sqlx_err)
    }
}
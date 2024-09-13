use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    MissingEnv(&'static str),
    RequestFail,
    CalendarUpdateFail,

    // Externals
    #[from]
    Diesel(diesel::result::Error),
    #[from]
    Env(std::env::VarError),
    #[from]
    Job(tokio_cron_scheduler::JobSchedulerError),
    #[from]
    Io(std::io::Error),
    #[from]
    Reqwest(reqwest::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

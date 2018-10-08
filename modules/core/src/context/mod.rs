use storage::Storage;

pub type Logger = dyn Fn(LogEvent, &str) -> () + Send + Sync + 'static;

#[derive(Debug, Clone, Serialize)]
#[repr(u8)]
pub enum LogEvent {
    Debug = 0,
    UserInfo = 1,
    Error = 2,
}

pub struct AppContext {
    pub(crate) storage: Storage,
    pub(crate) logger: Box<Logger>,
}

impl AppContext {
    pub fn log(&self, event: LogEvent, message: &str) {
        self.logger.as_ref()(event, message);
    }
}

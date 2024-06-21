use std::fmt::Result;

use chrono::prelude::*;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

pub struct Time;

impl Time {
    pub fn fmt(&self) -> String {
        format!("{}", Local::now().format("[ %d/%m/%Y - %X ] : "))
    }
}

impl FormatTime for Time {
    fn format_time(&self, w: &mut Writer<'_>) -> Result {
        write!(w, "{}", self.fmt())
    }
}

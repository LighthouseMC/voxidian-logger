#![feature(
    mpmc_channel,
    decl_macro
)]


use std::cell::LazyCell;
use std::sync::mpmc;
#[allow(unused)]
use std::sync::atomic::{ AtomicBool, Ordering };
use std::fmt;
use clap::Subcommand;
#[allow(unused)]
use chrono::{ Local, DateTime };
use crossterm as ct;
use ct::style::{ Stylize, StyledContent, Color as Colour };


pub macro trace ( $($tt:tt)* ) { log!( LogLevel::Trace , $($tt)* ) }
pub macro debug ( $($tt:tt)* ) { log!( LogLevel::Debug , $($tt)* ) }
pub macro info  ( $($tt:tt)* ) { log!( LogLevel::Info  , $($tt)* ) }
pub macro pass  ( $($tt:tt)* ) { log!( LogLevel::Pass  , $($tt)* ) }
pub macro warn  ( $($tt:tt)* ) { log!( LogLevel::Warn  , $($tt)* ) }
pub macro error ( $($tt:tt)* ) { log!( LogLevel::Error , $($tt)* ) }
pub macro fatal ( $($tt:tt)* ) { log!( LogLevel::Fatal , $($tt)* ) }

pub macro trace_once ( $($tt:tt)* ) { once!{ trace !( $($tt)* ) } }
pub macro debug_once ( $($tt:tt)* ) { once!{ debug !( $($tt)* ) } }
pub macro info_once  ( $($tt:tt)* ) { once!{ info  !( $($tt)* ) } }
pub macro pass_once  ( $($tt:tt)* ) { once!{ pass  !( $($tt)* ) } }
pub macro warn_once  ( $($tt:tt)* ) { once!{ warn  !( $($tt)* ) } }
pub macro error_once ( $($tt:tt)* ) { once!{ error !( $($tt)* ) } }
pub macro fatal_once ( $($tt:tt)* ) { once!{ fatal !( $($tt)* ) } }
pub macro once($expr:expr) { {
    static ALREADY_HIT : AtomicBool = AtomicBool::new(false);
    if (! ALREADY_HIT.swap(true, Relaxed)) {
        $expr;
    }
} }

macro log {
    ( $level:expr, $timestamp:expr => $($message:tt)* ) => { {
        let _ = LOGS.0.0.send(LogEntry {
            level    : $level,
            time_fmt : Into::<DateTime<Local>>::into($timestamp).format("%Y-%m-%d.%H:%M:%S%.9f").to_string(),
            message  : format!( $($message)* )
        });
    } },
    ( $level:expr, $($message:tt)* ) => {
        log!( $level, Local::now() => $($message)* )
    }
}


pub static LOGS : Logs = Logs::new();

pub struct Logs(LazyCell<(mpmc::Sender<LogEntry>, mpmc::Receiver<LogEntry>)>);
unsafe impl Sync for Logs { }
impl Logs {

    const fn new() -> Self { Self(LazyCell::new(|| mpmc::channel())) }

    pub fn copy_recv(&self) -> mpmc::Receiver<LogEntry> {
        self.0.1.clone()
    }

}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Pass,
    Warn,
    Error,
    Fatal
}
impl fmt::Debug for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", match self {
        Self::Trace => "TRACE",
        Self::Debug => "DEBUG",
        Self::Info  => "INFO",
        Self::Pass  => "PASS",
        Self::Warn  => "WARN",
        Self::Error => "ERROR",
        Self::Fatal => "FATAL",
    }) }
}
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub level    : LogLevel,
    pub time_fmt : String,
    pub message  : String
}
impl LogLevel {

    pub fn name(&self) -> &'static str { match (self) {
        Self::Trace => "TRACE",
        Self::Debug => "DEBUG",
        Self::Info  => "INFO ",
        Self::Pass  => "PASS ",
        Self::Warn  => "WARN ",
        Self::Error => "ERROR",
        Self::Fatal => "FATAL"
    } }

    pub fn stylise<'l>(&self, text : &'l str) -> StyledContent<&'l str> { match (self) {
        Self::Trace => text.dark_grey(),
        Self::Debug => text.grey(),
        Self::Info  => text.with(Colour::Rgb { r : 204, g : 229, b : 255 }),
        Self::Pass  => text.dark_green(),
        Self::Warn  => text.yellow(),
        Self::Error => text.red().bold(),
        Self::Fatal => text.with(Colour::Rgb { r : 255, g : 255, b : 255 }).on_red().bold()
    } }

}

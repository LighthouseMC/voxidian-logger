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
pub use crossterm as ct;
use ct::style::{ Stylize, StyledContent, Color as Colour };

#[allow(unused_imports, unused_macros)]
mod strcp {
    pub(crate) use const_str::{ split, concat, strip_suffix };
    pub(crate) macro remove_suffix( $s:expr, $suffix:expr ) { {
        const ORIGINAL : &'static str = $s;
        match (strip_suffix!( ORIGINAL, $suffix )) {
            Some(out) => out,
            None      => ORIGINAL
        } }
    }
}


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

#[allow(unused_macros)]
pub macro log {
    ( $level:expr, $timestamp:expr => $($message:tt)* ) => { {
        __private::log(
            strcp::concat!( env!("CARGO_PKG_NAME"), "/", strcp::remove_suffix!( strcp::split!( file!(), "src/" ).last().unwrap(), ".rs" ) ),
            line!(), column!(),
            $level,
            Into::<DateTime<Local>>::into($timestamp).format("%Y-%m-%d.%H:%M:%S%.9f").to_string(),
            format!( $($message)* )
        );
    } },
    ( $level:expr, $($message:tt)* ) => {
        log!( $level, Local::now() => $($message)* )
    }
}
#[doc(hidden)]
mod __private {
    use super::*;
    pub fn log(module : &'static str, line : u32, column : u32, level : LogLevel, time_fmt : String, message : String) {
        let _ = LOGS.0.0.send(LogEntry { module, line, column, level, time_fmt, message });
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
    pub module   : &'static str,
    pub line     : u32,
    pub column   : u32,
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





/*macro str_strip_suffix( $string:expr, $suffix:expr ) { {
    const ORIGINAL : &'static str = $string;
    const O_LEN    : usize        = ORIGINAL.len();
    const SUFFIX   : &'static str = $suffix;
    const S_LEN    : usize        = SUFFIX.len();
    const SPLICED  : SplicedStr   = str_splice!( ORIGINAL, (O_LEN - S_LEN)..O_LEN, "" );
    const OUTPUT   : &'static str = if (SPLICED.removed == SUFFIX) { SPLICED.output } else { ORIGINAL };
} }*/

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;

use fern::colors::{Color, ColoredLevelConfig};
use tauri::AppHandle;
use tauri::Manager;
use time::OffsetDateTime;

use crate::store::get_logging_level;

const MAX_LOG_FILE_SIZE: u64 = 5 * 1024 * 1024;

struct SizeLimitedWriter<W> {
    inner: W,
    remaining: u64,
}

impl<W> SizeLimitedWriter<W> {
    fn new(inner: W, remaining: u64) -> Self {
        Self { inner, remaining }
    }
}

impl<W: Write> Write for SizeLimitedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        if self.remaining == 0 {
            return Ok(buf.len());
        }

        let allowed = usize::try_from(self.remaining.min(buf.len() as u64)).unwrap_or(buf.len());
        let written = self.inner.write(&buf[..allowed])?;
        self.remaining = self.remaining.saturating_sub(written as u64);

        if written == allowed && allowed < buf.len() && self.remaining == 0 {
            // Report the whole message as consumed; bytes beyond the cap are intentionally dropped.
            Ok(buf.len())
        } else {
            Ok(written)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub fn set_up_logging(app_handle: &AppHandle) -> Result<(), anyhow::Error> {
    let default_level = match std::env::var("RQS_LOG") {
        Ok(r) => {
            println!("set_up_logging: level asked: {:?}", r);
            log::LevelFilter::from_str(&r).unwrap_or(log::LevelFilter::Debug)
        }
        Err(_) => match get_logging_level(app_handle) {
            Some(level_str) => {
                println!("set_up_logging: level from config: {:?}", level_str);
                log::LevelFilter::from_str(&level_str).unwrap_or(log::LevelFilter::Info)
            }
            None => {
                if cfg!(debug_assertions) {
                    log::LevelFilter::Trace
                } else {
                    log::LevelFilter::Info
                }
            }
        },
    };

    println!("set_up_logging: level: {:?}", default_level);
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    let dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "\x1B[2m{date}\x1b[0m {level: >5} \x1B[2m{target}:\x1b[0m {message}",
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors.color(record.level()),
                message = message,
            ));
        })
        .level(default_level)
        .level_for("mdns_sd", log::LevelFilter::Error)
        .level_for("polling", log::LevelFilter::Error)
        .level_for("neli", log::LevelFilter::Error)
        .level_for("bluez_async", log::LevelFilter::Error)
        .level_for("bluer", log::LevelFilter::Error)
        .level_for("async_io", log::LevelFilter::Error)
        .level_for("polling", log::LevelFilter::Error)
        .level_for("btleplug", log::LevelFilter::Error)
        .chain(std::io::stdout());

    if let Ok(path) = app_handle.path().app_log_dir() {
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        let app_name = &app_handle.package_info().name;
        let log_path = get_log_file_path(&path, app_name, MAX_LOG_FILE_SIZE)?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;
        let existing_size = file.metadata()?.len();
        let writer = SizeLimitedWriter::new(file, MAX_LOG_FILE_SIZE.saturating_sub(existing_size));

        dispatch
            .chain(Box::new(writer) as Box<dyn Write + Send>)
            .apply()?;
    } else {
        dispatch.apply()?;
    }

    debug!("Finished setting up logging! yay!");
    Ok(())
}

fn get_log_file_path(
    dir: &impl AsRef<Path>,
    file_name: &str,
    max_file_size: u64,
) -> Result<PathBuf, anyhow::Error> {
    let path = dir.as_ref().join(format!("{file_name}.log"));

    if path.exists() {
        let log_size = File::open(&path)?.metadata()?.len();
        if log_size > max_file_size {
            let to = dir.as_ref().join(format!(
                "{}_{}.log",
                file_name,
                OffsetDateTime::now_utc().unix_timestamp(),
            ));

            if to.is_file() {
                let mut to_bak = to.clone();
                let file_name = to_bak
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("rotated log path has no file name"))?
                    .to_string_lossy();
                to_bak.set_file_name(format!("{file_name}.bak"));
                std::fs::rename(&to, to_bak)?;
            }

            std::fs::rename(&path, to)?;
        }
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn size_limited_writer_caps_output() {
        let cursor = Cursor::new(Vec::<u8>::new());
        let mut writer = SizeLimitedWriter::new(cursor, 5);

        writer.write_all(b"1234567").unwrap();
        writer.write_all(b"89").unwrap();
        writer.flush().unwrap();

        assert_eq!(writer.inner.into_inner(), b"12345");
        assert_eq!(writer.remaining, 0);
    }

    #[test]
    fn size_limited_writer_preserves_short_output() {
        let cursor = Cursor::new(Vec::<u8>::new());
        let mut writer = SizeLimitedWriter::new(cursor, 8);

        writer.write_all(b"hello").unwrap();

        assert_eq!(writer.inner.into_inner(), b"hello");
        assert_eq!(writer.remaining, 3);
    }
}

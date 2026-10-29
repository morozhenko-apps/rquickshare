//! User-owned GTK language preference; never writes daemon or network settings.
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU32, AtomicU8, Ordering},
};

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

pub(super) const LOCALES: [&str; 20] = [
    "en", "ru", "es", "pt", "de", "fr", "it", "pl", "hi", "zh-Hant", "ja", "nl", "sv", "tr", "ko",
    "bn", "th", "id", "fil", "ar",
];

const APPLICATION_DIR: &str = "io.github.morozhenko_apps.RQuickShare";
const PREFERENCE_FILE: &str = "ui-language";
const MAX_LENGTH: u64 = 32;
static SELECTED: AtomicU8 = AtomicU8::new(0);
static NEXT_TEMP: AtomicU32 = AtomicU32::new(0);

/// Zero means the system language, 1..=20 are canonical explicit locales.
pub(super) fn selection() -> u8 {
    SELECTED.load(Ordering::Acquire)
}

pub(super) fn explicit_locale() -> Option<&'static str> {
    LOCALES.get(usize::from(selection().checked_sub(1)?)).copied()
}

fn parse(bytes: &[u8]) -> Option<u8> {
    let value = std::str::from_utf8(bytes).ok()?;
    let value = value.strip_suffix('\n').unwrap_or(value);
    if value == "system" {
        return Some(0);
    }
    LOCALES
        .iter()
        .position(|locale| *locale == value)
        .and_then(|index| u8::try_from(index + 1).ok())
}

fn config_file() -> io::Result<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .filter(|path| path.is_absolute())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No absolute config home"))?;
    Ok(base.join(APPLICATION_DIR).join(PREFERENCE_FILE))
}

fn load_at(path: &Path) -> io::Result<u8> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid UI language preference file",
        ));
    }
    let mut bytes = Vec::new();
    File::open(path)?.take(MAX_LENGTH + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Oversized UI language preference",
        ));
    }
    parse(&bytes).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid UI locale"))
}

/// Read only the GTK preference. Damaged files cannot activate a guessed language.
pub(super) fn initialize() {
    let selected = config_file()
        .and_then(|path| load_at(&path))
        .unwrap_or(0);
    SELECTED.store(selected, Ordering::Release);
}

fn prepare_directory(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        #[cfg(unix)]
        {
            let mut builder = fs::DirBuilder::new();
            builder.recursive(true).mode(0o700).create(parent)?;
        }
        #[cfg(not(unix))]
        fs::create_dir_all(parent)?;
        if fs::symlink_metadata(parent)?.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "UI preference directory must not be a symlink",
            ));
        }
    }
    Ok(())
}

fn save_at(path: &Path, selected: u8) -> io::Result<()> {
    let value = if selected == 0 {
        "system"
    } else {
        LOCALES
            .get(usize::from(selected - 1))
            .copied()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Unsupported UI locale"))?
    };
    prepare_directory(path)?;
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid existing UI preference",
            ));
        }
    }
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Missing preference directory")
    })?;
    let mut last_error = io::Error::new(io::ErrorKind::AlreadyExists, "Temporary name collision");
    for _ in 0..8 {
        let suffix = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let temp = parent.join(format!(".ui-language-{}-{suffix}.tmp", std::process::id()));
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        options.mode(0o600);
        match options.open(&temp) {
            Ok(mut output) => {
                let result = (|| {
                    output.write_all(value.as_bytes())?;
                    output.write_all(b"\n")?;
                    output.sync_all()?;
                    fs::rename(&temp, path)?;
                    Ok(())
                })();
                if result.is_err() {
                    let _ = fs::remove_file(&temp);
                }
                return result;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                last_error = error;
            }
            Err(error) => return Err(error),
        }
    }
    Err(last_error)
}

/// Persist first; change process language only after the atomic file replacement succeeds.
pub(super) fn save(selected: u8) -> io::Result<bool> {
    if selected > LOCALES.len() as u8 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported UI locale"));
    }
    if selected == selection() {
        return Ok(false);
    }
    save_at(&config_file()?, selected)?;
    SELECTED.store(selected, Ordering::Release);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_all_locale_options_when_parsed_then_only_explicit_canonical_ids_are_valid() {
        assert_eq!(parse(b"system\n"), Some(0));
        for (index, code) in LOCALES.iter().enumerate() {
            assert_eq!(parse(format!("{code}\n").as_bytes()), Some((index + 1) as u8));
        }
        for invalid in [
            "", "en-US", "pt-BR", "zh-Hans", "system\n\n", "xx", " ru",
            "system\r\n", "../en", "ru\0", "RU",
        ] {
            assert_eq!(parse(invalid.as_bytes()), None, "{invalid:?}");
        }
    }

    #[test]
    fn given_saved_preference_when_reloaded_then_exact_locale_roundtrips_without_daemon_files() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join(APPLICATION_DIR).join(PREFERENCE_FILE);
        for choice in 0..=LOCALES.len() as u8 {
            save_at(&path, choice).unwrap();
            assert_eq!(load_at(&path).unwrap(), choice);
        }
        assert!(path.exists());
        assert!(!temporary.path().join("native-settings.json").exists());
    }

    #[test]
    fn given_malformed_oversized_or_symlinked_preference_when_read_then_it_fails_closed() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("ui-language");
        fs::write(&path, vec![b'x'; 1024]).unwrap();
        assert_eq!(load_at(&path).unwrap_err().kind(), io::ErrorKind::InvalidData);
        fs::write(&path, b"pt-BR\n").unwrap();
        assert_eq!(load_at(&path).unwrap_err().kind(), io::ErrorKind::InvalidData);
        #[cfg(unix)]
        {
            let target = temporary.path().join("target");
            fs::write(&target, b"ru\n").unwrap();
            fs::remove_file(&path).unwrap();
            std::os::unix::fs::symlink(&target, &path).unwrap();
            assert_eq!(load_at(&path).unwrap_err().kind(), io::ErrorKind::InvalidData);
            assert!(save_at(&path, 2).is_err());
            assert_eq!(fs::read(&target).unwrap(), b"ru\n");
        }
    }

    #[test]
    fn given_invalid_selection_when_saving_then_no_file_is_written() {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("ui-language");
        assert_eq!(save_at(&path, 21).unwrap_err().kind(), io::ErrorKind::InvalidInput);
        assert!(!path.exists());
    }
}

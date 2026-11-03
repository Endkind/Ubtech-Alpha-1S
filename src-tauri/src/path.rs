use std::path::{Path, PathBuf};

/// Computes the directory of a path using standard Path methods.
/// If the path has a parent directory, returns it.
/// If the parent is empty (e.g. for a bare filename "main.py"), returns ".".
/// If there is no parent (e.g. root "/" or "C:\"), returns the path itself.
#[tauri::command]
pub fn path_dirname(path: String) -> String {
    if path.trim().is_empty() {
        return ".".to_string();
    }
    let p = Path::new(&path);
    match p.parent() {
        Some(parent) => {
            let s = parent.to_string_lossy();
            if s.is_empty() {
                ".".to_string()
            } else {
                s.to_string()
            }
        }
        None => path,
    }
}

/// Joins multiple path segments into a single normalized path using PathBuf::join.
#[tauri::command]
pub fn path_join(paths: Vec<String>) -> String {
    let mut iter = paths.into_iter().filter(|s| !s.trim().is_empty());
    let mut buf = match iter.next() {
        Some(first) => PathBuf::from(first),
        None => PathBuf::new(),
    };
    for p in iter {
        buf = buf.join(p);
    }
    buf.to_string_lossy().to_string()
}

/// Returns the last component of a path (filename or last directory name).
#[tauri::command]
pub fn path_basename(path: String) -> String {
    if path.trim().is_empty() {
        return "".to_string();
    }
    let p = Path::new(&path);
    match p.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => path,
    }
}

/// Checks whether the given path is absolute.
#[tauri::command]
pub fn path_is_absolute(path: String) -> bool {
    if path.trim().is_empty() {
        return false;
    }
    Path::new(&path).is_absolute()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_dirname() {
        assert_eq!(path_dirname("main.py".to_string()), ".");
        assert_eq!(path_dirname("".to_string()), ".");

        #[cfg(windows)]
        {
            assert_eq!(path_dirname(r"C:\Projekte\sdk\main.py".to_string()), r"C:\Projekte\sdk");
            assert_eq!(path_dirname(r"C:\main.py".to_string()), r"C:\");
            assert_eq!(path_dirname(r"C:\".to_string()), r"C:\");
            assert_eq!(path_dirname(r"\\server\share\folder\main.py".to_string()), r"\\server\share\folder");
            assert_eq!(path_dirname(r"\\server\share\main.py".to_string()), r"\\server\share\");
        }

        #[cfg(not(windows))]
        {
            assert_eq!(path_dirname("/home/user/sdk/main.py".to_string()), "/home/user/sdk");
            assert_eq!(path_dirname("/main.py".to_string()), "/");
            assert_eq!(path_dirname("/".to_string()), "/");
        }
    }

    #[test]
    fn test_path_join() {
        assert_eq!(path_join(vec![]), "");
        assert_eq!(path_join(vec!["   ".to_string()]), "");

        #[cfg(windows)]
        {
            assert_eq!(path_join(vec![r"C:\Projekte\sdk".to_string(), "main.py".to_string()]), r"C:\Projekte\sdk\main.py");
            assert_eq!(path_join(vec![r"C:\".to_string(), "main.py".to_string()]), r"C:\main.py");
            assert_eq!(path_join(vec![r"C:\".to_string(), ".venv".to_string()]), r"C:\.venv");
            assert_eq!(path_join(vec![r"C:\workspace".to_string(), "project".to_string(), "src".to_string()]), r"C:\workspace\project\src");
            assert_eq!(path_join(vec![r"\\server\share".to_string(), "main.py".to_string()]), r"\\server\share\main.py");
        }

        #[cfg(not(windows))]
        {
            assert_eq!(path_join(vec!["/home/user/sdk".to_string(), "main.py".to_string()]), "/home/user/sdk/main.py");
            assert_eq!(path_join(vec!["/".to_string(), "main.py".to_string()]), "/main.py");
            assert_eq!(path_join(vec!["/".to_string(), ".venv".to_string()]), "/.venv");
            assert_eq!(path_join(vec!["/workspace".to_string(), "project".to_string(), "src".to_string()]), "/workspace/project/src");
        }
    }

    #[test]
    fn test_path_basename() {
        assert_eq!(path_basename("".to_string()), "");
        assert_eq!(path_basename("main.py".to_string()), "main.py");

        #[cfg(windows)]
        {
            assert_eq!(path_basename(r"C:\Projekte\sdk\main.py".to_string()), "main.py");
            assert_eq!(path_basename(r"C:\Projekte\sdk".to_string()), "sdk");
        }

        #[cfg(not(windows))]
        {
            assert_eq!(path_basename("/home/user/sdk/main.py".to_string()), "main.py");
            assert_eq!(path_basename("/home/user/sdk".to_string()), "sdk");
        }
    }

    #[test]
    fn test_path_is_absolute() {
        assert!(!path_is_absolute("".to_string()));
        assert!(!path_is_absolute("relative/path".to_string()));

        #[cfg(windows)]
        {
            assert!(path_is_absolute(r"C:\Projekte\sdk".to_string()));
            assert!(path_is_absolute(r"\\server\share".to_string()));
            assert!(!path_is_absolute(r"relative\path".to_string()));
        }

        #[cfg(not(windows))]
        {
            assert!(path_is_absolute("/home/user/sdk".to_string()));
            assert!(!path_is_absolute("relative/path".to_string()));
        }
    }
}

// Language: en-GB. Included inside main.rs's test module; no live registry operations.

struct StartupSettingsDirectory(PathBuf);

impl StartupSettingsDirectory {
    fn new() -> Self {
        let suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "monitor-startup-page-{}-{suffix}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for StartupSettingsDirectory {
    fn drop(&mut self) {
        if self.0.starts_with(std::env::temp_dir())
            && self
                .0
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("monitor-startup-page-"))
        {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

fn assert_unrelated_settings_save_ignores_startup(entry: AppResult<Option<String>>) {
    let directory = StartupSettingsDirectory::new();
    let mut app = application();
    app.dir = directory.0.clone();
    store::save(&app.dir, &app.settings).unwrap();
    app.startup_registered = None;
    app.startup_choice_dirty = false;
    let original_startup = app.settings.preferences.start_with_windows;
    let original_dark = app.settings.preferences.dark;
    let path = app.monitors[0].path.clone();
    app.settings_draft.preferences.dark = !original_dark;
    app.settings_draft
        .aliases
        .insert(path.clone(), "Example monitor".into());
    app.settings_draft.fallback = Some(path.clone());
    let (saved, reads, writes) = startup::with_test_registry(entry, || app.save_settings_page());
    assert!(saved);
    assert_eq!(
        (reads, writes),
        (0, 0),
        "Unrelated settings must not access the startup registry"
    );
    let (persisted, _) = store::load(&app.dir).unwrap();
    assert_eq!(persisted.preferences.dark, !original_dark);
    assert_eq!(persisted.preferences.start_with_windows, original_startup);
    assert_eq!(
        persisted.aliases.get(&path).map(String::as_str),
        Some("Example monitor")
    );
    assert_eq!(persisted.fallback.as_deref(), Some(path.as_str()));
    assert_eq!(app.startup_registered, None);
}

fn assert_requested_startup_save_preserves_unusable_entry(entry: AppResult<Option<String>>) {
    let directory = StartupSettingsDirectory::new();
    let mut app = application();
    app.dir = directory.0.clone();
    store::save(&app.dir, &app.settings).unwrap();
    let before = std::fs::read(app.dir.join("settings.json")).unwrap();
    app.startup_registered = None;
    app.startup_choice = !app.settings.preferences.start_with_windows;
    app.startup_choice_dirty = true;
    app.settings_draft.preferences.dark = !app.settings.preferences.dark;
    let (saved, reads, writes) = startup::with_test_registry(entry, || app.save_settings_page());
    assert!(!saved);
    assert_eq!((reads, writes), (1, 0));
    assert_eq!(
        std::fs::read(app.dir.join("settings.json")).unwrap(),
        before
    );
    assert!(
        app.startup_choice_dirty,
        "Failed explicit startup change must remain unsaved"
    );
}

#[test]
fn unrelated_settings_save_succeeds_with_foreign_startup_entry() {
    assert_unrelated_settings_save_ignores_startup(Ok(Some(
        r#""C:\SyntheticOther\MonitorSwitcher.exe""#.into(),
    )));
}

#[test]
fn unrelated_settings_save_succeeds_with_unreadable_startup_entry() {
    assert_unrelated_settings_save_ignores_startup(Err(
        "Synthetic unreadable startup entry.".into()
    ));
}

#[test]
fn requested_startup_save_preserves_foreign_entry_and_settings() {
    assert_requested_startup_save_preserves_unusable_entry(Ok(Some(
        r#""C:\SyntheticOther\MonitorSwitcher.exe""#.into(),
    )));
}

#[test]
fn requested_startup_save_preserves_unreadable_entry_and_settings() {
    assert_requested_startup_save_preserves_unusable_entry(Err(
        "Synthetic unreadable startup entry.".into(),
    ));
}

//! The Dock icon (macOS). CC Same is in the Dock, and in ⌘-Tab, while its window is open; with the
//! window closed it lives in the menu bar, unless it is set to stay in the Dock.

/// Put the app in the Dock or take it out.
#[cfg(target_os = "macos")]
pub fn show(visible: bool) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy as Policy};

    let Some(mtm) = MainThreadMarker::new() else { return };
    let app = NSApplication::sharedApplication(mtm);
    let policy = if visible { Policy::Regular } else { Policy::Accessory };
    if app.activationPolicy() != policy {
        app.setActivationPolicy(policy);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn show(_visible: bool) {}

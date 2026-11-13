<div lang="en-GB">

# Monitor Switcher 1.1.0

Monitor profiles and display controls for Windows 10 and Windows 11 (x64).

## Download and run

Download **MonitorSwitcher.exe**, save it in a permanent folder and open it.
No installer, .NET runtime, WebView2 or NirSoft monitor tool is required.
Fonts, icon, application runtime and licence texts are embedded. Windows supplies
the system libraries and graphics driver.

If an older version is running, choose **Exit** from its notification-area menu
before replacing it. Compatible profiles and preferences are retained locally
in `%APPDATA%\WorkMonitorSwitcher`.

## What's new

- Profiles can remember a **Main display**. Choose it when saving a profile or
  change it on the Profiles tab, then apply the profile. Choose **Keep current
  main if possible** to leave the choice unspecified.
- Click or right-click a numbered monitor in the diagram to **Disable display**.
  Connected disabled monitors offer **Enable display** below the diagram.
  At least one display must remain enabled.
- The notification-area menu now offers **Turn off all displays**, using the
  same two-second countdown as the main window. Mouse or keyboard input wakes
  the displays; this sleep action does not change profiles or desktop membership.
- Monitor-card buttons, diagram menus, name editors and information controls
  keep their physical-monitor identity when displays are added, removed or
  reordered. A click cannot transfer to a different display after hotplug.
- Automatic detection cannot interrupt the sleep countdown. Conflicting manual
  actions cancel it with a clear message rather than silently dropping it.
- Stronger topology, display-mode and rollback verification; primary changes
  preserve relative positions and rotation while rebasing the desktop origin.
- Safer settings recovery, profile replacement, startup registration and shutdown
  handling. Unrelated settings changes do not overwrite another startup command.
- Updated dependency and release checks, including regenerated build checksums.

Windows Display Settings remains authoritative for arrangement, orientation and
resolution. Profiles select enabled monitors and optionally a main display;
they do not restore a stale saved arrangement. Unsafe overlapping or disconnected
layouts are rejected rather than silently repositioning displays. If rollback
cannot be verified, further display changes are locked until Windows Display
Settings has been checked and the app restarted.

Older version 1 settings are read without being overwritten on launch. Saving
changes writes version 2 settings. Rolling back to a version 1-only app also
requires a compatible local settings backup.

## Important

This executable is **not Authenticode-signed**. Windows SmartScreen may show an
unknown publisher or unrecognised app warning. Use the official release download
and compare its SHA-256 with **SHA256SUMS.txt**. No security settings are changed
by the app or need to be disabled.

Extended desktops are supported; mirrored/duplicated displays are not. A profile
containing a physically disconnected monitor fails safely rather than disabling
the remaining displays. Switching and physical reconnection remain dependent on
Windows and the graphics driver. No automatic update downloader is included.

The release passed 90 Rust regression tests, 24 synthetic release-check tests,
formatting, build-output and privacy checks. The dependency scan found no
unreviewed advisories; three previously reviewed non-Windows maintenance notices
remain. These checks are not a guarantee that no private information or bugs exist.

Automated tests do not change displays; physical switching, reconnection, display
wake and graphics-driver behaviour require hardware validation.

</div>

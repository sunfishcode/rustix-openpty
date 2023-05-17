use std::fs::File;
use std::io::{self, Read, Write};

#[test]
fn test_openpty_basic() -> io::Result<()> {
    let pty = rustix_openpty::openpty(None, None)?;

    let mut controller = File::from(pty.controller);
    let mut user = File::from(pty.user);

    // The '\x04' is Ctrl-D, the default EOF control code.
    controller.write_all(b"Hello, world!\n\x04")?;

    let mut s = String::new();
    user.read_to_string(&mut s)?;

    assert_eq!(s, "Hello, world!\n");
    Ok(())
}

#[test]
fn test_openpty_with_winsize() -> io::Result<()> {
    let winsize = rustix_openpty::rustix::termios::Winsize {
        ws_col: 42,
        ws_row: 43,
        ws_xpixel: 44,
        ws_ypixel: 45,
    };
    let pty = rustix_openpty::openpty(None, Some(&winsize))?;

    let found = rustix::termios::tcgetwinsize(&pty.user)?;

    assert_eq!(winsize.ws_col, found.ws_col);
    assert_eq!(winsize.ws_row, found.ws_row);
    assert_eq!(winsize.ws_xpixel, found.ws_xpixel);
    assert_eq!(winsize.ws_ypixel, found.ws_ypixel);

    Ok(())
}

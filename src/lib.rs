use command_line::{command_exists, execute_command};
use std::process;

#[macro_use]
extern crate derive_builder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        match NotificationBuilder::default()
            .title("TEST NOTIFICATION")
            .subtitle("Subtitle")
            .message("This is \"the\" message.")
            .sound("Pop")
            .open("https://google.com")
            .build() {
            Ok(notification) => notification.notify(),
            Err(err) => { dbg!("{}", err); }
        }
        // you should see a desktop notification
    }
}


#[derive(Default, Builder, Debug)]
pub struct Notification<'notification> {
    #[builder(setter(into))]
    pub title: &'notification str,
    #[builder(setter(into))]
    pub subtitle: &'notification str,
    #[builder(setter(into))]
    pub message: &'notification str,
    #[builder(setter(strip_option), default)]
    pub sound: Option<&'notification str>,
    #[builder(setter(strip_option), default)]
    pub open: Option<&'notification str>,
}

impl Notification<'_> {
    pub fn notify(&self) {
        _notify(&self)
    }
}


fn _notify(notification: &Notification) {
    let title = notification.title;
    let subtitle = notification.subtitle;
    let message = notification.message;
    let open_str = match notification.open {
            Some(s) => s,
            None => "",
        };

    if cfg!(target_os = "macos") {
        let sound_str = match notification.sound {
            Some(s) => s,
            None => "default",
        };
        terminal_notifier_command(title, subtitle, message, sound_str, open_str);
    } else {
        notify_send_command(title, subtitle, message, open_str);
    }
}


const TERMINAL_NOTIFIER_UNSAFE_CHARS: [char; 2] = ['[', ']'];

fn terminal_notifier_command(title: &str, subtitle: &str, message: &str, sound: &str, open: &str) {
    // check terminal-notifier is installed
    if !command_exists("terminal-notifier -h") {
        println!("terminal-notifier is not available. Is it installed?");
        process::exit(1);
    }

    // escape chars not supported by terminal-notifier
    let mut safe_message = message.to_owned();
    for c in TERMINAL_NOTIFIER_UNSAFE_CHARS {
        safe_message = safe_message.replace(c, "")
    }

    // build MacOS terminal-notifier command line
    let mut notification_str = format!(
        "-title \"{title}\" \
         -subtitle \"{subtitle}\" \
         -message \"{safe_message}\" \
         -sound \"{sound}\""
    );
    if open.len() > 0 {
        notification_str = format!("{notification_str} -open \"{open}\"")
    }

    // execute the command
    execute_command(&format!("terminal-notifier {notification_str}"), true);
}

fn notify_send_command(title: &str, subtitle: &str, message: &str, url: &str) {
    // check notify-send is installed
    if !command_exists("notify-send -h") {
        println!("notify-send is not available. Is it installed?");
        process::exit(1);
    }

    let mut safe_message = message.to_owned();
    safe_message = safe_message.replace('"', "'");

    // build linux command line arguments
    let mut notification_str = format!("\"{title} ({subtitle})\" \"{safe_message}\"");
    if url.len() > 0 {
        notification_str.push_str(&format!(" {url}"))
    }

    // execute command
    execute_command(&format!("notify-send {notification_str}"), true);
}

// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{process::exit, thread::sleep, time::Duration as StdDuration};

use tauri_winrt_notification::{Duration, Sound, Toast, ToastNotification};
use windows::core::IInspectable;

fn main() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .on_activated(handle_notification_click)
        .show()
        .expect("unable to send notification");
    println!("Waiting 10 seconds for the notification to be clicked...");
    sleep(StdDuration::from_secs(10));
    println!("The notification wasn't clicked!");
}

fn handle_notification_click(
    toast: &Option<ToastNotification>,
    object: &Option<IInspectable>,
) -> windows::core::Result<()> {
    println!("You've clicked me!");
    dbg!(toast);
    dbg!(object);
    exit(0);
}

// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::sync::{Barrier, OnceLock};

use tauri_winrt_notification::{Duration, Sound, Toast, ToastNotification};
use windows::core::IInspectable;

static GATE: OnceLock<Barrier> = OnceLock::new();

fn main() {
    GATE.set(Barrier::new(2)).unwrap();

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .on_activated(handle_notification_click)
        .show()
        .expect("unable to send notification");
    println!("Waiting for the notification to be clicked...");

    GATE.get().unwrap().wait();
}

fn handle_notification_click(
    toast: &Option<ToastNotification>,
    object: &Option<IInspectable>,
) -> windows::core::Result<()> {
    println!("You've clicked me!");
    dbg!(toast);
    dbg!(object);
    GATE.get().unwrap().wait();
    Ok(())
}

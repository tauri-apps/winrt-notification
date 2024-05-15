---
"tauri-winrt-notification": minor
---

Add support for adding buttons using `Toast::add_button`. This also comes with a change to `Toast::on_activated` wich will now take an `Option<String>` argument, containing which button was pressed if any.
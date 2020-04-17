fn main() {
    let mut connection = zbus::Connection::new_session()
        .map_err(|e| {
            println!("error: {}", e);

            e
        })
    .unwrap();

    let method = match std::env::args().nth(1) {
        Some(s) => {
            if s == "+" {
                "StepUp"
            } else if s == "-" {
                "StepDown"
            } else {
                panic!("Expected either '+' or '-' argument. Got: {}", s);
            }
        },
        None => "StepUp",
    };

    let reply = connection
        .call_method(
            Some("org.gnome.SettingsDaemon.Power"),
            "/org/gnome/SettingsDaemon/Power",
            Some("org.gnome.SettingsDaemon.Power.Screen"),
            method,
            &(),
        )
        .unwrap();

    let (percent, _)  = reply.body::<(u32, &str)>().unwrap();
    println!("New level: {}%", percent);
}

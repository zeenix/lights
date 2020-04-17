fn main() {
    let mut connection = zbus::Connection::new_session()
        .map_err(|e| {
            println!("error: {}", e);

            e
        })
    .unwrap();

    let reply = connection
        .call_method(
            Some("org.gnome.SettingsDaemon.Power"),
            "/org/gnome/SettingsDaemon/Power",
            Some("org.gnome.SettingsDaemon.Power.Screen"),
            "StepDown",
            &(),
        )
        .unwrap();

    let (percent, _)  = reply.body::<(u32, &str)>().unwrap();
    println!("New level: {}%", percent);
}

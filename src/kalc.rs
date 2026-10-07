use base::config::keys;
use hbb_common::config;

const APP_NAME: &str = "KALCDESK";

// Filled at build time from the KALC_SERVER / KALC_KEY repository secrets.
const SERVER: Option<&str> = option_env!("KALC_SERVER");
const SERVER_KEY: Option<&str> = option_env!("KALC_KEY");

pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();

    {
        let mut local = config::OVERWRITE_LOCAL_SETTINGS.write().unwrap();
        local.insert(keys::OPTION_THEME.to_owned(), "dark".to_owned());
        local.insert(keys::OPTION_ENABLE_CHECK_UPDATE.to_owned(), "N".to_owned());
    }

    {
        let mut settings = config::OVERWRITE_SETTINGS.write().unwrap();
        settings.insert(keys::OPTION_ALLOW_AUTO_UPDATE.to_owned(), "N".to_owned());
        if let (Some(server), Some(key)) = (non_empty(SERVER), non_empty(SERVER_KEY)) {
            settings.insert(
                keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned(),
                server.to_owned(),
            );
            settings.insert(keys::OPTION_RELAY_SERVER.to_owned(), server.to_owned());
            settings.insert(keys::OPTION_KEY.to_owned(), key.to_owned());
        }
    }

    let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
    for k in [
        keys::OPTION_HIDE_SERVER_SETTINGS,
        keys::OPTION_HIDE_PROXY_SETTINGS,
        keys::OPTION_HIDE_WEBSOCKET_SETTINGS,
        keys::OPTION_HIDE_HELP_CARDS,
    ] {
        builtin.insert(k.to_owned(), "Y".to_owned());
    }
}

fn non_empty(v: Option<&'static str>) -> Option<&'static str> {
    v.map(str::trim).filter(|s| !s.is_empty())
}

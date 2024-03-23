use bitsync_jwt::JwtClaims;
use leptos::{
    create_effect, create_local_resource, create_trigger, provide_context, signal_prelude::*,
    use_context, Resource, Trigger,
};
use leptos_use::{use_cookie, utils::FromToStringCodec};
use material_colors::{argb_from_hex, theme_from_source_color, utils::theme::Theme};

use crate::api::{
    private::query::{MeQuery, User},
    ApiError, GraphQlSendQueryOperationHelper,
};

static JWT_STORAGE_KEY: &str = "JWT";

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginState {
    Invalid,
    NotSet,
    Set(JwtClaims),
}

#[derive(Debug)]
pub struct ColorPalette {
    pub theme: Theme,
}

impl Clone for ColorPalette {
    fn clone(&self) -> Self {
        Self {
            theme: theme_from_source_color(self.theme.source, Vec::new()),
        }
    }
}

impl PartialEq for ColorPalette {
    fn eq(&self, other: &Self) -> bool {
        other.theme.source == self.theme.source
    }
}

#[derive(Clone)]
struct GlobalLoginStorage {
    login_state: Memo<LoginState>,
    login: Signal<Option<String>>,
    set_login: WriteSignal<Option<String>>,
    logout: Trigger,
    current_user: Resource<Option<String>, Option<Result<User, ApiError>>>,
}

impl GlobalLoginStorage {
    fn new() -> Self {
        let (login, set_login) = use_cookie::<String, FromToStringCodec>(JWT_STORAGE_KEY);

        let login_state = create_memo(move |_| {
            let login_token = login.get();

            match login_token {
                None => LoginState::NotSet,
                Some(value) if value.is_empty() => LoginState::Invalid,
                Some(value) => match JwtClaims::decode(&value) {
                    Ok(claims) => LoginState::Set(claims),
                    Err(_) => LoginState::Invalid,
                },
            }
        });

        let logout_trigger = create_trigger();

        create_effect(move |_| {
            logout_trigger.track();
            set_login.set(None);
        });

        let current_user_resource = create_local_resource(
            move || login.get(),
            move |login| async move {
                match login {
                    None => None,
                    Some(_) => Some(MeQuery::send(()).await.map(|me_query| me_query.me)),
                }
            },
        );

        Self {
            login_state,
            login,
            set_login,
            logout: logout_trigger,
            current_user: current_user_resource,
        }
    }
}

pub fn provide_login_storage() {
    provide_context(GlobalLoginStorage::new())
}

pub fn use_login_state() -> Memo<LoginState> {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    login_storage.login_state
}

pub fn use_login_token() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    (login_storage.login, login_storage.set_login)
}

pub fn use_logout() -> Trigger {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    login_storage.logout
}

pub fn use_current_user() -> Resource<Option<String>, Option<Result<User, ApiError>>> {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    login_storage.current_user
}

pub fn use_color_palette() -> Memo<ColorPalette> {
    let login_storage =
        use_context::<GlobalLoginStorage>().expect("GlobalLoginStorage is initialized");

    create_memo(move |_| match login_storage.current_user.get() {
        Some(Some(Ok(user))) => match user
            .color_palette
            .and_then(|color_palette_string| argb_from_hex(color_palette_string).ok())
        {
            Some(argb_color) => ColorPalette {
                theme: theme_from_source_color(argb_color, Vec::new()),
            },
            None => ColorPalette {
                theme: theme_from_source_color([1, 2, 3, 4], Vec::new()),
            },
        },
        _ => ColorPalette {
            theme: theme_from_source_color([1, 2, 3, 4], Vec::new()),
        },
    })
}

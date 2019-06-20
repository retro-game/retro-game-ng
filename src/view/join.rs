use crate::service::join::ErrorFlags;
use crate::validation::{email, password, user_name};
use maud::{html, DOCTYPE};

pub fn join(error_flags: ErrorFlags) -> String {
    let markup = html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Join - Retro Game" }
            link href="/static/skins/EpicBlue/style.css" rel="stylesheet";
        }
        body {
            div.container {
                div {
                    div {
                        h1 { "retro-game-ng" }
                        form action="/join" method="post" {
                            table {
                                tr {
                                    @if error_flags.is_empty() {
                                        th colspan="2" { "Join" }
                                    } @else {
                                        th colspan="3" { "Join" }
                                    }
                                }
                                tr {
                                    td {
                                        label for="email" { "Email" }
                                    }
                                    td {
                                        input#email name="email" type="email" autocomplete="off" maxlength=(email::MAX_LENGTH) autofocus? required?;
                                    }
                                    @if !error_flags.is_empty() {
                                        td {
                                            @if error_flags.contains(ErrorFlags::EMAIL_TOO_LONG) {
                                                p { "Email is too long" }
                                            }
                                            @if error_flags.contains(ErrorFlags::EMAIL_WRONG_FORMAT) {
                                                p { "Email has a wrong format" }
                                            }
                                            @if error_flags.contains(ErrorFlags::EMAIL_EXISTS) {
                                                p { "This email already exists" }
                                            }
                                        }
                                    }
                                }
                                tr {
                                    td {
                                        label for="name" { "Nickname" }
                                    }
                                    td {
                                        input#name name="name" minlength=(user_name::MIN_LENGTH) maxlength=(user_name::MAX_LENGTH) pattern=(user_name::PATTERN) required?;
                                    }
                                    @if !error_flags.is_empty() {
                                        td {
                                            @if error_flags.contains(ErrorFlags::NAME_TOO_SHORT) {
                                                p { "Nickname is too short" }
                                            }
                                            @if error_flags.contains(ErrorFlags::NAME_TOO_LONG) {
                                                p { "Nickname is too long" }
                                            }
                                            @if error_flags.contains(ErrorFlags::NAME_WRONG_FORMAT) {
                                                p { "Nickname has a wrong format" }
                                            }
                                            @if error_flags.contains(ErrorFlags::NAME_EXISTS) {
                                                p { "This nickname already exists" }
                                            }
                                        }
                                    }
                                }
                                tr {
                                    td {
                                        label for="password" { "Password" }
                                    }
                                    td {
                                        input#password name="password" type="password" minlength=(password::MIN_LENGTH) maxlength=(password::MAX_LENGTH) required?;
                                    }
                                    @if !error_flags.is_empty() {
                                        td {
                                            @if error_flags.contains(ErrorFlags::PASSWORD_TOO_SHORT) {
                                                p { "Password is too short" }
                                            }
                                            @if error_flags.contains(ErrorFlags::PASSWORD_TOO_LONG) {
                                                p { "Password is too long" }
                                            }
                                        }
                                    }
                                }
                                tr {
                                    td {
                                        label for="password-confirm" { "Confirm password" }
                                    }
                                    td {
                                        input#password-confirm name="password-confirm" type="password" minlength=(password::MIN_LENGTH) maxlength=(password::MAX_LENGTH) required?;
                                    }
                                    @if !error_flags.is_empty() {
                                        td {
                                            @if error_flags.contains(ErrorFlags::PASSWORDS_DO_NOT_MATCH) {
                                                p { "Passwords don't match" }
                                            }
                                        }
                                    }
                                }
                                tr {
                                    @if error_flags.is_empty() {
                                        td colspan="2" {
                                            button { "Join" }
                                        }
                                    } @else {
                                        td colspan="3" {
                                            button { "Join" }
                                        }
                                    }
                                }
                            }
                        }
                        p {
                            { "or sign in " }
                            a href="/" {
                                strong { "here" }
                            }
                            { "." }
                        }
                    }
                }
            }
        }
    };
    markup.into()
}

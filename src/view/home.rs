use maud::{html, DOCTYPE};

pub fn home(sign_in_error: bool, joined: bool) -> String {
    let markup = html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Sign in - Retro Game" }
            link href="/static/skins/EpicBlue/style.css" rel="stylesheet";
        }
        body {
            div.container {
                div {
                    div {
                        h1 { "retro-game-ng" }
                        @if sign_in_error {
                            table {
                                tr {
                                    th { "Error" }
                                }
                                tr {
                                    td { "Bad credentials" }
                                }
                            }
                        } @else if joined {
                            table {
                                tr {
                                    th { "Joined" }
                                }
                                tr {
                                    td { "Account created successfully, you can sign in now!" }
                                }
                            }
                        }
                        form action="/sign-in" method="post" {
                            table {
                                tr {
                                    th colspan="2" { "Sign in" }
                                }
                                tr {
                                    td {
                                        label for="email" { "Email" }
                                    }
                                    td {
                                        input#email name="email" type="email" autocomplete="off" autofocus? required?;
                                    }
                                }
                                tr {
                                    td {
                                        label for="password" { "Password" }
                                    }
                                    td {
                                        input#password name="password" type="password" required?;
                                    }
                                }
                                tr {
                                    td colspan="2" {
                                        button { "Sign in" }
                                    }
                                }
                            }
                        }
                        p {
                            { "or join " }
                            a href="/join" {
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

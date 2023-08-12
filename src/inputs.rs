macro_rules! define_keys {
    ($($name:ident: $code:expr, $str:expr;)*) => {
        $(
            pub const $name: Key = Key::new($code, $str);
        )*
    };
}

define_keys! {
    KEY_ESCAPE: 27, "Escape";
    KEY_SPACE: 32, "Space";

    KEY_UP: 65, "Up";
    KEY_DOWN: 66, "Down";
    KEY_RIGHT: 67, "Right";
    KEY_LEFT: 68, "Left";

    KEY_F1: 80, "F1";
    KEY_F2: 81, "F2";
    KEY_F3: 82, "F3";
    KEY_F4: 83, "F4";
    KEY_F5: 84, "F5";
    KEY_F6: 85, "F6";
    KEY_F7: 86, "F7";
    KEY_F8: 87, "F8";
    KEY_F9: 88, "F9";
    KEY_F10: 89, "F10";
    KEY_F11: 90, "F11";
    KEY_F12: 91, "F12";

    KEY_0: 48, "0";
    KEY_1: 49, "1";
    KEY_2: 50, "2";
}
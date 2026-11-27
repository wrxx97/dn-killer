use std::fmt;
use std::str::FromStr;

use anyhow::{Result, anyhow, bail, ensure};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{VK_0, VK_9, VK_A, VK_F1, VK_F12, VK_Z};

pub const HOTKEY_PLACEHOLDER: &str = "例如 Ctrl+F7、Alt+F4，留空禁用";
const INVALID_HOTKEY: &str = "请输入 F1–F12，或用 Ctrl、Alt、Shift、Win 搭配功能键、字母或数字";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Modifiers {
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
    pub win: bool,
}

impl Modifiers {
    const NONE: Self = Self {
        control: false,
        alt: false,
        shift: false,
        win: false,
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hotkey {
    key: u16,
    modifiers: Modifiers,
}

impl Hotkey {
    pub const ALT_F4: Self = Self {
        key: VK_F1 + 3,
        modifiers: Modifiers {
            alt: true,
            ..Modifiers::NONE
        },
    };

    pub const F7: Self = Self {
        key: VK_F1 + 6,
        modifiers: Modifiers::NONE,
    };

    pub fn supports_key(key: u16) -> bool {
        matches!(key, VK_F1..=VK_F12 | VK_A..=VK_Z | VK_0..=VK_9)
    }

    pub fn is_function_key(self) -> bool {
        (VK_F1..=VK_F12).contains(&self.key)
    }

    pub fn from_virtual_key(key: u16, modifiers: Modifiers) -> Option<Self> {
        let shortcut = Self { key, modifiers };
        (Self::supports_key(key) && (shortcut.is_function_key() || modifiers != Modifiers::NONE))
            .then_some(shortcut)
    }

    pub fn from_key_name(name: &str, modifiers: Modifiers) -> Option<Self> {
        let name = name.to_ascii_uppercase();
        let key = match name.as_bytes() {
            [key @ (b'A'..=b'Z' | b'0'..=b'9')] => u16::from(*key),
            _ => {
                let function = name.strip_prefix('F')?.parse::<u16>().ok()?;
                if !(1..=12).contains(&function) {
                    return None;
                }
                VK_F1 + function - 1
            }
        };
        Self::from_virtual_key(key, modifiers)
    }

    pub fn parse_optional(text: &str) -> Result<Option<Self>> {
        let text = text.trim();
        if text.is_empty() {
            Ok(None)
        } else {
            text.parse().map(Some)
        }
    }
}

impl FromStr for Hotkey {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self> {
        let mut parts = text.split('+').map(str::trim);
        let key = parts.next_back().unwrap_or_default();
        let mut modifiers = Modifiers::default();
        for part in parts {
            let modifier = match part.to_ascii_lowercase().as_str() {
                "ctrl" | "control" => &mut modifiers.control,
                "alt" => &mut modifiers.alt,
                "shift" => &mut modifiers.shift,
                "win" => &mut modifiers.win,
                _ => bail!("{INVALID_HOTKEY}"),
            };
            ensure!(!*modifier, "快捷键中有重复的修饰键");
            *modifier = true;
        }
        Self::from_key_name(key, modifiers).ok_or_else(|| anyhow!("{INVALID_HOTKEY}"))
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (enabled, label) in [
            (self.modifiers.control, "Ctrl+"),
            (self.modifiers.alt, "Alt+"),
            (self.modifiers.shift, "Shift+"),
            (self.modifiers.win, "Win+"),
        ] {
            if enabled {
                f.write_str(label)?;
            }
        }
        if self.is_function_key() {
            write!(f, "F{}", self.key - VK_F1 + 1)
        } else {
            write!(f, "{}", char::from(self.key as u8))
        }
    }
}

impl Serialize for Hotkey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StoredHotkey {
    LegacyFunction(u8),
    Label(String),
}

impl StoredHotkey {
    fn into_hotkey<E: de::Error>(self) -> Result<Hotkey, E> {
        match self {
            Self::LegacyFunction(number @ 1..=12) => Ok(Hotkey {
                key: VK_F1 + u16::from(number) - 1,
                modifiers: Modifiers::NONE,
            }),
            Self::LegacyFunction(_) => Err(E::custom(INVALID_HOTKEY)),
            Self::Label(label) => label.parse().map_err(E::custom),
        }
    }
}

impl<'de> Deserialize<'de> for Hotkey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        StoredHotkey::deserialize(deserializer)?.into_hotkey()
    }
}

pub fn deserialize_exit_hotkey<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Hotkey>, D::Error> {
    match Option::<StoredHotkey>::deserialize(deserializer)? {
        Some(StoredHotkey::LegacyFunction(6)) => Ok(Some(Hotkey::ALT_F4)),
        Some(stored) => stored.into_hotkey().map(Some),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_round_trips_a_combination() -> Result<()> {
        let shortcut: Hotkey = "shift + ctrl + r".parse()?;
        assert_eq!(shortcut.to_string(), "Ctrl+Shift+R");
        assert_eq!(
            shortcut,
            serde_json::from_str(&serde_json::to_string(&shortcut)?)?
        );
        assert_eq!("alt+f4".parse::<Hotkey>()?, Hotkey::ALT_F4);
        assert_eq!(Hotkey::parse_optional("  ")?, None);
        for invalid in [
            "R",
            "Ctrl+",
            "Tip+F4",
            "Ctrl+Ctrl+R",
            "Win+F13",
            "Ctrl+Shift",
        ] {
            assert!(invalid.parse::<Hotkey>().is_err(), "{invalid}");
        }
        Ok(())
    }
}

use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};

use crate::hotkey::{Hotkey, deserialize_exit_hotkey};

pub const MAX_SECONDS: u64 = 86_400;
const CONFIG_VERSION: u32 = 1;

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    version: u32,
    pub tasks: Vec<TaskDefinition>,
    pub settings: Settings,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TaskDefinition {
    pub id: u64,
    pub title: String,
    pub duration: u64,
    pub hotkey: Option<Hotkey>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct Settings {
    pub process_name: String,
    #[serde(deserialize_with = "deserialize_exit_hotkey")]
    pub exit_hotkey: Option<Hotkey>,
    pub voice_enabled: bool,
    pub voice_lead: u64,
    pub voice_template: String,
    pub opacity: u8,
    pub font_color: u32,
    pub topmost: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            tasks: vec![TaskDefinition {
                id: 1,
                title: "龙之谷倒计时".into(),
                duration: 15,
                hotkey: Some(Hotkey::F7),
            }],
            settings: Settings::default(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            process_name: "DragonNest_x64.exe".into(),
            exit_hotkey: Some(Hotkey::ALT_F4),
            voice_enabled: true,
            voice_lead: 10,
            voice_template: "{title}还有{left}秒".into(),
            opacity: 50,
            font_color: 0xffffff,
            topmost: true,
        }
    }
}

impl Config {
    pub fn next_task_id(&self) -> Result<u64> {
        self.tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .context("任务编号已用完")
    }

    fn validate(&self) -> Result<()> {
        ensure!(
            self.version == CONFIG_VERSION,
            "配置版本 {} 不受支持",
            self.version
        );
        let mut task_ids = HashSet::new();
        for task in &self.tasks {
            ensure!(
                task.id != 0 && task_ids.insert(task.id),
                "任务编号无效或重复"
            );
            task.validate()?;
        }
        self.settings.validate()
    }
}

impl TaskDefinition {
    pub fn validate(&self) -> Result<()> {
        ensure!(!self.title.trim().is_empty(), "请输入任务名称");
        ensure!(
            (1..=MAX_SECONDS).contains(&self.duration),
            "时长须为 1–{MAX_SECONDS} 秒"
        );
        Ok(())
    }
}

impl Settings {
    pub fn validate(&self) -> Result<()> {
        let name = self.process_name.trim();
        ensure!(
            self.exit_hotkey.is_none() || !name.is_empty(),
            "启用退出快捷键时须填写进程名"
        );
        ensure!(
            !name.contains(['\\', '/', ':', '\0']),
            "进程名只填文件名，例如 DragonNest_x64.exe"
        );
        ensure!(
            (1..=MAX_SECONDS).contains(&self.voice_lead),
            "提醒时间须为 1–{MAX_SECONDS} 秒"
        );
        ensure!(
            !self.voice_enabled || !self.voice_template.trim().is_empty(),
            "请输入语音提示模板"
        );
        ensure!(self.opacity <= 100, "不透明度须为 0–100");
        ensure!(self.font_color <= 0xffffff, "颜色须为六位十六进制值");
        Ok(())
    }
}

pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    pub fn new() -> Result<Self> {
        let local = dirs::data_local_dir().context("找不到 Windows 本地应用数据目录")?;
        Ok(Self {
            path: local.join("dn-killer").join("config.json"),
        })
    }

    pub fn load(&self) -> Result<Config> {
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Config::default()),
            Err(error) => return Err(error).context("读取配置失败"),
        };
        let config: Config = serde_json::from_slice(&bytes).context("配置 JSON 无效")?;
        config.validate()?;
        Ok(config)
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        config.validate()?;
        let directory = self.path.parent().context("配置路径无效")?;
        fs::create_dir_all(directory).context("创建配置目录失败")?;
        let mut file = tempfile::NamedTempFile::new_in(directory).context("创建临时配置失败")?;
        serde_json::to_writer_pretty(&mut file, config).context("写入配置失败")?;
        file.write_all(b"\n").context("写入配置失败")?;
        file.as_file().sync_all().context("刷新配置文件失败")?;
        file.persist(&self.path).context("替换配置文件失败")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_replaces_config_in_a_new_directory() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let store = ConfigStore {
            path: directory.path().join("new").join("config.json"),
        };
        let mut config = store.load()?;
        store.save(&config)?;
        config.tasks[0].title = "技能冷却".into();
        config.settings.opacity = 75;
        store.save(&config)?;
        let saved = store.load()?;
        assert_eq!(saved.tasks[0].title, "技能冷却");
        assert_eq!(saved.settings.opacity, 75);
        Ok(())
    }

    #[test]
    fn rejects_unsupported_version() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let store = ConfigStore {
            path: directory.path().join("config.json"),
        };
        let config = Config {
            version: 2,
            ..Config::default()
        };
        fs::write(&store.path, serde_json::to_vec(&config)?)?;
        assert!(store.load().is_err());
        Ok(())
    }

    #[test]
    fn migrates_only_the_legacy_default_exit_key() -> Result<()> {
        let mut old = serde_json::to_value(Config::default())?;
        old["tasks"][0]["hotkey"] = 6.into();
        old["settings"]["exit_hotkey"] = 6.into();
        let config: Config = serde_json::from_value(old.clone())?;
        assert_eq!(config.tasks[0].hotkey, Some("F6".parse()?));
        assert_eq!(config.settings.exit_hotkey, Some(Hotkey::ALT_F4));

        for (stored, expected) in [
            (9.into(), Some("F9".parse()?)),
            ("F6".into(), Some("F6".parse()?)),
            (serde_json::Value::Null, None),
        ] {
            old["settings"]["exit_hotkey"] = stored;
            let config: Config = serde_json::from_value(old.clone())?;
            assert_eq!(config.settings.exit_hotkey, expected);
        }
        Ok(())
    }
}

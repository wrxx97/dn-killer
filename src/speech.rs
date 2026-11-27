use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Instant;

use anyhow::{Context, Result, ensure};
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::DataReader;
use windows::Win32::Media::Audio::{PlaySoundW, SND_MEMORY, SND_NODEFAULT, SND_SYNC};
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize, RoUninitialize};
use windows::core::{HSTRING, PCWSTR};

pub struct Reminder {
    pub text: String,
    pub seconds: u64,
    pub deadline: Instant,
}

pub fn start(results: async_channel::Sender<Result<()>>) -> Result<Sender<Reminder>> {
    let (sender, queue) = mpsc::channel();
    thread::Builder::new()
        .name("dn-killer-speech".into())
        .spawn(move || {
            if let Err(error) = run(queue, &results) {
                let _ = results.try_send(Err(error));
            }
        })
        .context("创建语音线程失败")?;
    Ok(sender)
}

fn run(queue: Receiver<Reminder>, results: &async_channel::Sender<Result<()>>) -> Result<()> {
    // SAFETY: This dedicated thread owns the WinRT objects and balances initialization on exit.
    unsafe { RoInitialize(RO_INIT_MULTITHREADED) }.context("初始化 Windows 语音环境失败")?;
    let _apartment = Apartment;
    let synth = SpeechSynthesizer::new().context("创建 Windows 语音合成器失败")?;
    let voices = SpeechSynthesizer::AllVoices().context("读取 Windows 语音列表失败")?;
    let chinese_voice = voices
        .into_iter()
        .filter_map(|voice| {
            let language = voice.Language().ok()?.to_string().to_ascii_lowercase();
            (language == "zh" || language.starts_with("zh-"))
                .then_some((language != "zh-cn", voice))
        })
        .min_by_key(|(other_locale, _)| *other_locale);
    let use_chinese = if let Some((_, voice)) = chinese_voice {
        synth.SetVoice(&voice).context("选择中文语音失败")?;
        true
    } else {
        false
    };

    for reminder in queue {
        if Instant::now() >= reminder.deadline {
            continue;
        }
        let text = if use_chinese || reminder.text.is_ascii() {
            reminder.text
        } else {
            format!("{} seconds remaining.", reminder.seconds)
        };
        let result = speak(&synth, &text, reminder.deadline).context("播放倒计时语音失败");
        if results.try_send(result).is_err() {
            break;
        }
    }
    Ok(())
}

fn speak(synth: &SpeechSynthesizer, text: &str, deadline: Instant) -> Result<()> {
    let stream = synth
        .SynthesizeTextToStreamAsync(&HSTRING::from(text))?
        .join()?;
    let size = u32::try_from(stream.Size()?).context("语音音频过大")?;
    let reader = DataReader::CreateDataReader(&stream.GetInputStreamAt(0)?)?;
    ensure!(
        reader.LoadAsync(size)?.join()? == size,
        "语音音频读取不完整"
    );
    let mut wave = vec![0; size as usize];
    reader.ReadBytes(&mut wave)?;
    ensure!(
        wave.starts_with(b"RIFF") && wave.get(8..12) == Some(b"WAVE"),
        "Windows 返回了未知的音频格式"
    );

    if Instant::now() < deadline {
        // SAFETY: SND_MEMORY takes WAV bytes and SND_SYNC keeps the buffer borrowed until playback ends.
        let played = unsafe {
            PlaySoundW(
                PCWSTR(wave.as_ptr().cast()),
                None,
                SND_MEMORY | SND_NODEFAULT | SND_SYNC,
            )
        };
        ensure!(played.as_bool(), "Windows 音频设备未能播放语音");
    }
    Ok(())
}

struct Apartment;

impl Drop for Apartment {
    fn drop(&mut self) {
        // SAFETY: It is dropped on the same thread, after all WinRT objects are released.
        unsafe { RoUninitialize() };
    }
}

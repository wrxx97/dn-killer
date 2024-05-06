use futures::Future;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};
use tts::Tts;

pub struct TtsFeature {
    text: String,
    state: Arc<Mutex<State>>,
}

struct State {
    waker: Option<Waker>,
    speak_state: SpeakState,
    tts: Tts,
}

#[derive(PartialEq)]
enum SpeakState {
    Init,
    Speaking,
    Done,
}

impl TtsFeature {
    pub fn new(text: String) -> Self {
        Self {
            text,
            state: Arc::new(Mutex::new(State {
                tts: Tts::default().unwrap(),
                waker: None,
                speak_state: SpeakState::Init,
            })),
        }
    }
}

impl Future for TtsFeature {
    type Output = Result<(), tts::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();

        if guard.speak_state == SpeakState::Done {
            return Poll::Ready(Ok(()));
        }

        if guard.speak_state == SpeakState::Init {
            println!("speak start");
            guard.waker = Some(cx.waker().clone());
            guard.speak_state = SpeakState::Speaking;

            let tts = &guard.tts;
            let state_cloned = Arc::clone(&self.state);
            let text = self.text.clone();

            tts.on_utterance_end(Some(Box::new(move |_| {
                let mut guard = state_cloned.lock().unwrap();
                guard.speak_state = SpeakState::Done;
                if let Some(waker) = guard.waker.take() {
                    waker.wake_by_ref();
                }
            })))
            .expect("error");
            println!("{:?}", &tts.voices());
            let _ = guard.tts.speak(text, false);
        }

        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

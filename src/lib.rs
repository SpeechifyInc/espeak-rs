extern crate napi;

use espeakng_sys::*;
use napi::tokio::task::spawn;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_short};

use napi_derive::napi;

#[napi]
pub struct EspeakAddon {
  pub voice_name: String,
  pub options: u32,
  pub buffer_len: i32,
  has_initialized: bool,
}

#[napi]
impl EspeakAddon {
  #[napi]
  pub fn new(voice_name: String, options: u32, buffer_len: i32) -> Self {
    EspeakAddon {
      voice_name,
      options,
      buffer_len,
      has_initialized: false,
    }
  }
  #[napi]
  pub fn default() -> Self {
    let espeak = EspeakAddon {
      voice_name: "en-us".to_string(),
      buffer_len: 1000,
      options: espeakINITIALIZE_PHONEME_EVENTS,
      has_initialized: false,
    };
    espeak
  }

  #[napi]
  pub fn initialize(&mut self) {
    assert!(!self.has_initialized);
    let output: espeak_AUDIO_OUTPUT = espeak_AUDIO_OUTPUT_AUDIO_OUTPUT_RETRIEVAL;
    let path: *const c_char = std::ptr::null();
    let voice_name_cstr =
      CString::new(self.voice_name.clone()).expect("Failed to convert &str to CString");
    let voice_name = voice_name_cstr.as_ptr();

    // Returns: sample rate in Hz, or -1 (EE_INTERNAL_ERROR).
    let _sample_rate =
      unsafe { espeak_Initialize(output, self.buffer_len, path, self.options as i32) };

    unsafe {
      espeak_SetVoiceByName(voice_name as *const c_char);
      espeak_SetSynthCallback(Some(synth_callback));
    };

    self.has_initialized = true;

    let text_cstr = CString::new("").expect("Failed to convert &str to CString");
    let position = 0u32;
    let position_type: espeak_POSITION_TYPE = 0;
    let end_position = 0u32;
    let flags = espeakCHARS_AUTO & espeak_PARAMETER_espeakSILENCE;
    let identifier = std::ptr::null_mut();
    let user_data = std::ptr::null_mut();

    // By calling synth here, espeak creates a global translator that is required by TextToPhonemes later
    unsafe {
      espeak_Synth(
        text_cstr.as_ptr() as *const c_void,
        self.buffer_len as size_t,
        position,
        position_type,
        end_position,
        flags,
        identifier,
        user_data,
      );
    }
  }

  #[napi]
  pub async fn text_to_phonemes(&self, text: String) -> String {
    let text_cstr = CString::new(text).expect("Failed to convert &str to CString");
    unsafe {
      let phonemes = espeak_TextToPhonemes(
        &mut (text_cstr.as_ptr() as *const c_void),
        espeakCHARS_AUTO as i32,
        espeakPHONEMES_IPA as i32,
      );
      let result = CStr::from_ptr(phonemes).to_str().unwrap();
      result.to_string()
    }
  }

  pub fn text_to_phonemes_sync(&self, text: String) -> String {
    let text_cstr = CString::new(text).expect("Failed to convert &str to CString");
    unsafe {
      let phonemes = espeak_TextToPhonemes(
        &mut text_cstr.as_ptr().cast() as *mut *const std::ffi::c_void,
        espeakCHARS_UTF8 as i32,
        espeakPHONEMES_IPA as i32,
      );
      let result = CStr::from_ptr(phonemes).to_string_lossy().to_string();
      result
    }
  }
}

/// Callback returns: 0=continue synthesis,  1=abort synthesis.
unsafe extern "C" fn synth_callback(
  _wav: *mut c_short,
  _sample_count: c_int,
  _events: *mut espeak_EVENT,
) -> c_int {
  0
}

#[napi]
pub struct EspeakRunner {
  _addon: EspeakAddon,
}

#[napi]
impl EspeakRunner {
  #[napi(constructor)]
  pub fn new() -> EspeakRunner {
    let mut addon = EspeakAddon::default();
    addon.initialize();
    EspeakRunner { _addon: addon }
  }
  #[napi]
  pub async fn run_phoneme_task(text: String) -> String {
    let thread_join_handle = spawn(async move {
      let espeak = EspeakAddon::default();
      let res = espeak.text_to_phonemes_sync(text);
      res
    });

    thread_join_handle.await.unwrap()
  }
}

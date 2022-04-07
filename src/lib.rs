#![allow(non_upper_case_globals)]

extern crate napi;

use espeakng_sys::*;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_short};

use napi_derive::napi;

#[napi]
pub struct EspeakAddon {
    pub voice_name: String,
    pub options: u32,
    pub buffer_len: i32,
    has_initialized: bool
}

#[napi]
impl EspeakAddon {
    #[napi]
    pub fn new(voice_name: String, options: u32, buffer_len: i32) -> Self {
        EspeakAddon {
            voice_name,
            options,
            buffer_len,
            has_initialized: false
        }
    }
    #[napi]
    pub fn default() -> Self {
        let mut espeak = EspeakAddon {
            voice_name: "en-us".to_string(),
            buffer_len: 1000,
            options: espeakINITIALIZE_PHONEME_EVENTS,
            has_initialized: false
        };
        espeak.initialize();
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

        let text_cstr = CString::new("test").expect("Failed to convert &str to CString");
        let position = 0u32;
        let position_type: espeak_POSITION_TYPE = 0;
        let end_position = 0u32;
        let flags = espeakCHARS_AUTO & espeak_PARAMETER_espeakSILENCE;
        let identifier = std::ptr::null_mut();
        let user_data = std::ptr::null_mut();

        let mut phoneme_buffer: Vec<i8> = Vec::with_capacity(1000); // arbitrary capacity of 1000;
        let cap = phoneme_buffer.capacity();
        let memstream = unsafe {
            espeakng_sys::open_memstream(&mut phoneme_buffer.as_mut_ptr(), &mut (cap as u64))
        };

        // By calling synth here, espeak creates a global translator that is required by TextToPhonemes later
        unsafe {
            espeak_SetPhonemeTrace(2, memstream);
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
        assert!(self.has_initialized);
        let text_cstr = CString::new(text).expect("Failed to convert &str to CString");

        let mut phoneme_buffer: Vec<i8> = Vec::with_capacity(1000); // arbitrary capacity of 1000;
        let cap = phoneme_buffer.capacity();
        let memstream = unsafe {
            espeakng_sys::open_memstream(&mut phoneme_buffer.as_mut_ptr(), &mut (cap as u64))
        };

        unsafe {
            espeak_SetPhonemeTrace(2, memstream);
            let phonemes = espeak_TextToPhonemes(
                &mut (text_cstr.as_ptr() as *const c_void),
                espeakCHARS_AUTO as i32,
                espeakPHONEMES_IPA as i32,
            );
            let result = CStr::from_ptr(phonemes).to_str().unwrap();
            result.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use markov::Chain;
    use std::process::Command;
    use std::{
        fs,
        io::{self, BufRead, BufReader},
    };

    use futures::executor::block_on;
    #[test]
    fn does_not_panic() {
        let espeak = EspeakAddon::default();
        let result = block_on(espeak.text_to_phonemes("of the".to_string()));
        println!("{}", result);
    }

    fn setup_markov() -> io::Result<Chain<String>> {
        let entries = fs::read_dir("./testfiles")?
            .map(|result| result.map(|item| item.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        let mut chain: Chain<String> = Chain::new();
        entries.iter().for_each(|item| {
            let file = fs::File::open(item).unwrap();
            let buf_reader = BufReader::new(file);
            buf_reader.lines().for_each(|line| {
                if let Ok(content) = line {
                    chain.feed_str(&content);
                }
            })
        });

        Ok(chain)
    }

    fn call_espeak_cli(text: &str) -> String {
        let result = Command::new("sh")
            .arg("-c")
            .arg(format!("espeak-ng \"{}\" -q -x --ipa -v en-us", text))
            .output()
            .expect("cli execution failed")
            .stdout;
        let output = String::from_utf8_lossy(&result).into_owned();
        output
    }
    #[test]
    fn markov_test_once() {
        let chain = setup_markov().expect("Failed to create markov chain");
        let espeak = EspeakAddon::default();
        let random_input = chain.generate_str();
        let result = block_on(espeak.text_to_phonemes(random_input.clone()));
        let result_from_cli = call_espeak_cli(&random_input);
        println!(
            "Input: {} \nPhonemes: {}CLI: {}",
            &random_input, &result, &result_from_cli
        );
        assert!(result.eq(&result_from_cli) || result_from_cli.is_empty())
    }

    #[test]
    fn markov_test_lot() {
        const n: u32 = 10;
        (1..n).for_each(|_| markov_test_once());
    }
}

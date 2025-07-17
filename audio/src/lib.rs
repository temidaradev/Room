use sdl2::mixer::{Chunk, Music, Channel, DEFAULT_CHANNELS};
use wad::WadFile;

pub struct AudioManager {
    _mixer_context: sdl2::mixer::Sdl2MixerContext,
    sound_effects: std::collections::HashMap<String, Chunk>,
    current_music: Option<Music<'static>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG)?;

        // Initialize mixer with reasonable defaults
        sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)?;
        sdl2::mixer::allocate_channels(16);

        Ok(AudioManager {
            _mixer_context: mixer_context,
            sound_effects: std::collections::HashMap::new(),
            current_music: None,
        })
    }

    pub fn load_sound_effects(&mut self, wad: &WadFile) -> Result<(), Box<dyn std::error::Error>> {
        let sound_names = ["DSPISTOL", "DSSHOTGN", "DSPLASMA", "DSBFG", "DSRLAUNC"];

        for sound_name in &sound_names {
            if let Some(lump) = wad.find_lump(sound_name) {
                let sound_data = self.convert_doom_sound_to_wav(&lump.data)?;
                let chunk = Chunk::from_raw_buffer(sound_data.into_boxed_slice())?;
                self.sound_effects.insert(sound_name.to_string(), chunk);
            }
        }

        Ok(())
    }

    pub fn play_sound_3d(&self, sound_name: &str, player_pos: (f64, f64), sound_pos: (f64, f64)) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(chunk) = self.sound_effects.get(sound_name) {
            let distance = ((sound_pos.0 - player_pos.0).powi(2) + (sound_pos.1 - player_pos.1).powi(2)).sqrt();

            // Calculate volume based on distance
            let volume = (255.0 / (1.0 + distance / 100.0)) as i32;
            let volume = volume.max(0).min(255);

            // Calculate panning based on relative position
            let angle = (sound_pos.1 - player_pos.1).atan2(sound_pos.0 - player_pos.0);
            let pan = ((angle.sin() + 1.0) * 127.0) as u8;

            let channel = Channel::all().play(chunk, 0)?;
            channel.set_volume(volume);
            channel.set_panning(255 - pan, pan)?;
        }

        Ok(())
    }

    fn convert_doom_sound_to_wav(&self, doom_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Doom sounds have a simple header format
        if doom_data.len() < 8 {
            return Err("Invalid Doom sound data".into());
        }

        let sample_rate = u16::from_le_bytes([doom_data[2], doom_data[3]]);
        let sample_count = u32::from_le_bytes([doom_data[4], doom_data[5], doom_data[6], doom_data[7]]);

        // Convert to standard WAV format for SDL2
        let mut wav_data = Vec::new();

        // WAV header
        wav_data.extend_from_slice(b"RIFF");
        wav_data.extend_from_slice(&(36 + sample_count).to_le_bytes());
        wav_data.extend_from_slice(b"WAVE");
        wav_data.extend_from_slice(b"fmt ");
        wav_data.extend_from_slice(&16u32.to_le_bytes());
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // PCM
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // Mono
        wav_data.extend_from_slice(&(sample_rate as u32).to_le_bytes());
        wav_data.extend_from_slice(&(sample_rate as u32).to_le_bytes());
        wav_data.extend_from_slice(&1u16.to_le_bytes());
        wav_data.extend_from_slice(&8u16.to_le_bytes()); // 8-bit
        wav_data.extend_from_slice(b"data");
        wav_data.extend_from_slice(&sample_count.to_le_bytes());

        // Sound data (skip header)
        wav_data.extend_from_slice(&doom_data[8..]);

        Ok(wav_data)
    }
}
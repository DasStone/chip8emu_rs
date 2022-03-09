// https://docs.rs/sdl2/0.30.0/sdl2/audio/index.html

use sdl2::{audio::{AudioDevice, AudioCallback, AudioSpecDesired}, Sdl};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume * if self.phase <= 0.5 { 1. } else { -1. };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct SoundHandler {
    device: AudioDevice<SquareWave>,
    muted: bool,
}

impl SoundHandler {
    pub fn new(sdl_context: &Sdl, muted: bool) -> SoundHandler {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            //freq: Some(44100),
            freq: Some(11025),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.5
            }
        }).unwrap();

        SoundHandler {
            device: device,
            muted: muted,
        }
    }

    pub fn resume(&self) {
        if !self.muted {
            self.device.resume();
        }
    }

    pub fn pause(&self) {
        self.device.pause();
    }
}

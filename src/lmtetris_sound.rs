#![allow(clippy::precedence)]

use std::thread;

use assert_no_alloc::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, FromSample, SizedSample};
use fundsp::prelude64::*;
use midly::{MidiMessage, Smf, Timing, TrackEventKind};
use rand::seq;
use std::sync::mpsc;

enum MsgSound {
    Swirl,
    Combo,
    Smash,
    Tmove,
    Pause,
}

struct SoundPlayer {}

pub struct Sound {
    fs: i32,
    tx: mpsc::Sender<MsgSound>,
}

impl Sound {
    pub fn new(fs: i32) -> Self {
        let (tx, rx) = mpsc::channel();
        start(rx);
        Sound { fs, tx }
    }
    pub fn start(&self) {}
    pub fn pause(&mut self) {
        self.tx.send(MsgSound::Pause).unwrap();
    }
    pub fn swirl(&mut self) {
        self.tx.send(MsgSound::Swirl).unwrap();
    }
    pub fn combo(&mut self) {
        self.tx.send(MsgSound::Combo).unwrap();
    }
    pub fn smash(&mut self) {
        self.tx.send(MsgSound::Smash).unwrap();
    }
    pub fn tmove(&mut self) {
        self.tx.send(MsgSound::Tmove).unwrap();
    }
}

fn start(rx: mpsc::Receiver<MsgSound>) {
    thread::spawn(move || {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Failed to find a default output device");
        let supported_config = device.default_output_config().unwrap();
        let mut config = supported_config.config();
        config.buffer_size = BufferSize::Fixed(256);
        match supported_config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), &rx).unwrap(),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), &rx).unwrap(),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), &rx).unwrap(),
            _ => panic!("Unsupported format"),
        }
    });
}
fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    rx: &mpsc::Receiver<MsgSound>,
) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let smf = Smf::parse(include_bytes!("../assets/melody.mid"))?;

    let mut tps = 0.001;

    if let Timing::Timecode(fps, subframe) = smf.header.timing {
        tps = 1.0 / (fps.as_f32() * subframe as f32);
    }

    let sample_rate = config.sample_rate as f64;
    let channels = config.channels as usize;

    let (snoop0, snoop_backend0) = snoop(32768);
    let (snoop1, snoop_backend1) = snoop(32768);

    let freqs = [shared(0.0), shared(0.0), shared(0.0)];
    let volumes = [shared(0.0), shared(0.0), shared(0.0)];
    let filter_freq = shared(2000.0);
    let current_time = shared(0.0);

    let mut c0 = (var(&freqs[0]) >> poly_saw()) * var(&volumes[0]);
    let mut c1 = (var(&freqs[1]) >> poly_saw()) * var(&volumes[1]);
    let mut c2 = (var(&freqs[2]) >> poly_saw()) * var(&volumes[2]);
    let mut sequencer = Sequencer::new(0, 1, ReplayMode::None);
    let sequencer_backend = sequencer.backend();
    let mut c = Net::wrap(Box::new(sequencer_backend));
    let mut c = (c | var(&filter_freq)) >> lowpass_q(1.0);
    let mut c_r = c.clone() >> chorus(12345, 0.05, 0.03, 0.3);
    let mut c_l = c >> chorus(12345, 0.05, 0.03, 0.3);
    let mut c = (c_l | c_r) >> reverb_stereo(30.0, 1.0, 0.9);

    c.set_sample_rate(sample_rate);

    let mut backend = BlockRateAdapter::new(Box::new(c.backend()));
    let mut next_value = move || backend.get_stereo();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;
    let mut iter = smf.tracks[0].iter().cycle();
    let mut notes = [None; 12];
    let mut id = [None; 12];
    loop {
        if let Ok(msg) = rx.try_recv() {
            match msg {
                MsgSound::Combo => {
                    // c = (c | c) >> reverb_stereo(30.0, 3.0, 0.8) >> join::<U2>();
                }
                MsgSound::Smash => {
                    //c = c >> shape_fn(|x| x * 5.0 * x.signum()) >> clip();
                }
                MsgSound::Swirl => {}
                _ => (),
            }
        }
        let mut ticks = 10.0;
        if let Some(track_event) = iter.next() {
            if let TrackEventKind::Midi { channel, message } = track_event.kind {
                if let MidiMessage::NoteOn { key, vel } = message {
                    let n_new = key.as_int();
                    for n in &mut notes {
                        if n.is_none() {
                            *n = Some(n_new);
                            break;
                        }
                    }
                }
                if let MidiMessage::NoteOff { key, vel } = message {
                    let n_stop = key.as_int();
                    for note in &mut notes {
                        if let Some(n) = note {
                            if *n == n_stop {
                                *note = None;
                            }
                        }
                    }
                }
            }
            ticks = track_event.delta.as_int() as f32;

            for (i, n) in notes.iter().enumerate() {
                if let Some(n) = n
                    && id[i] == None
                {
                    let mut tone =
                        Net::wrap(Box::new(saw_hz(midi_hz(*n as f32)) * 0.2 >> dcblock()));
                    tone.ping(false, AttoHash::new(123453123));
                    id[i] = Some(sequencer.push_relative(
                        0.0,
                        f64::INFINITY,
                        Fade::Smooth,
                        0.02,
                        0.2,
                        Box::new(tone),
                    ));
                } else {
                    if let Some(s_id) = id[i] {
                        sequencer.edit_relative(s_id, 0.2, 0.2);
                        id[i] = None;
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(ticks * tps));
    }
}
fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f32, f32))
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}

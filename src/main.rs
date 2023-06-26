use core::time::Duration;
use rodio::{OutputStream, source::Source};

fn main() {

    //A wave table is an array in memory, which contains 1 period of the waveform
    // we want to play out through our oscillator.
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    /*
        We calculate the value of the sine waveform for arguments linearly increasing from
        0 to 2π to calculate the sine value for argument.

        By populating the wave_table array with the calculated sine values,
         we generate a single cycle of a sine waveform within the specified range.
         This waveform can then be used as a basis for creating more complex sounds in music synthesis applications.
     */
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));

    /*
        We want to write a wavetable oscillator: an object that iterates over a specific wave table
        with speed dictated by the frequency of the tone it should output.
        That object needs to store the sampling rate, the wave table, current index into the wave table,
        and the frequency-dependent index increment.
     */

    struct WavetableOscillator {
        sample_rate: u32,
        wave_table: Vec<f32>,
        index: f32,
        index_increment: f32,
    }

    impl WavetableOscillator {
        fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
            return WavetableOscillator {
                sample_rate: sample_rate,
                wave_table: wave_table,
                index: 0.0,
                index_increment: 0.0,
            };
        }

        /*
            Sets the frequency of the wavetable oscillator by calculating the index_increment value.
            The index_increment determines how quickly the oscillator moves through the wavetable
            to generate the waveform.

            Setting the frequency is essential because it determines the pitch of the sound produced by the wavetable oscillator.
             A higher frequency will result in a higher-pitched sound, while a lower frequency will produce a lower-pitched sound. By adjusting the frequency dynamically, we can generate different musical notes and create melodies

            The set_frequency function allows us to conveniently update the frequency parameter
            of the oscillator and adjust its output in real-time.
         */
        fn set_frequency(&mut self, frequency: f32) {
            self.index_increment = frequency * self.wave_table.len() as f32
                / self.sample_rate as f32;
        }

        /*
            Generating a sample consists of linear interpolation of the wave table values according to the index value and incrementing the index.
         */

        fn get_sample(&mut self) -> f32 {
            let sample = self.lerp();
            self.index += self.index_increment;
            self.index %= self.wave_table.len() as f32;
            return sample;
        }

        fn lerp(&self) -> f32 {
            let truncated_index = self.index as usize;
            let next_index = (truncated_index + 1) % self.wave_table.len();

            let next_index_weight = self.index - truncated_index as f32;
            let truncated_index_weight = 1.0 - next_index_weight;

            return truncated_index_weight * self.wave_table[truncated_index]
                + next_index_weight * self.wave_table[next_index];
        }
    }

    impl Iterator for WavetableOscillator {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            return Some(self.get_sample());
        }
    }

    impl Source for WavetableOscillator {
        fn channels(&self) -> u16 {
            return 1;
        }

        fn sample_rate(&self) -> u32 {
            return self.sample_rate;
        }

        fn current_frame_len(&self) -> Option<usize> {
            return None;
        }

        fn total_duration(&self) -> Option<Duration> {
            return None;
        }
    }
}

#[macro_use]
extern crate vst;
extern crate time;

use std::sync::Arc;
use vst::buffer::AudioBuffer;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

#[derive(Default)]
struct BasicPlugin;

#[derive(Default)]
struct GainEffect {
    params: Arc<GainEffectParameters>,
}

struct GainEffectParameters {
    amplitude: AtomicFloat,
}

impl Default for GainEffectParameters {
    fn default() -> GainEffectParameters {
        GainEffectParameters {
            amplitude: AtomicFloat::new(0.5),
        }
    }
}

impl Plugin for GainEffect {
    fn new(_host: HostCallback) -> Self {
        // Note that controls will always return a value from 0 - 1.
        // Setting a default to 0.5 means it's halfway up.
        GainEffect {
            params: Arc::new(GainEffectParameters::default()),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Andrew VST Experiments".to_string(),
            vendor: "Rust DSP".to_string(),
            unique_id: 243723167,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // Read the amplitude from the parameter object
        let amplitude = self.params.amplitude.get();
        // First, we destructure our audio buffer into an arbitrary number of
        // input and output buffers.  Usually, we'll be dealing with stereo (2 of each)
        // but that might change.
        for (input_buffer, output_buffer) in buffer.zip() {
            // Next, we'll loop through each individual sample so we can apply the amplitude
            // value to it.
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = *input_sample * amplitude;
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}


impl PluginParameters for GainEffectParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.amplitude.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.amplitude.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("Hello {:.2}", (self.amplitude.get() - 0.5) * 2f32),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Amplitude",
            _ => "",
        }
        .to_string()
    }
}

plugin_main!(GainEffect);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

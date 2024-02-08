use noise::{NoiseFn, OpenSimplex};
use std::convert::From;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum NoiseFnTypes {
    OpenSimplex,
}

pub enum AugmentedNoiseFnOperation {
    None,
    AddScalar(f64),
    AddNoiseFn(AugmentedNoiseFn),
    MultiplyScalar(f64),
    MultiplyNoiseFn(AugmentedNoiseFn),
}

pub struct TerrainGenerator {
    pub noise_function: AugmentedNoiseFn,
    pub operations: Option<Vec<AugmentedNoiseFnOperation>>,
}

impl TerrainGenerator {
    pub fn new(
        noise_function: AugmentedNoiseFn,
        operations: Option<Vec<AugmentedNoiseFnOperation>>,
    ) -> Self {
        return Self {
            noise_function,
            operations,
        };
    }

    pub fn sample(&self, x: f64, y: f64) -> f64 {
        use AugmentedNoiseFnOperation::*;
        let mut total = self.noise_function.sample(x, y);

        if let Some(operations) = &self.operations {
            for operation in operations {
                match operation {
                    MultiplyNoiseFn(g) => total *= g.sample(x, y),
                    MultiplyScalar(s) => total *= s,
                    AddNoiseFn(g) => total += g.sample(x, y),
                    AddScalar(s) => total += s,
                    None => {}
                }
            }
        }

        return total;
    }
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        return Self::new(AugmentedNoiseFn::default(), None);
    }
}

pub struct AugmentedNoiseFn {
    noise_function: Box<dyn NoiseFn<f64, 2>>,
    amplitude: f64,
    x_frequency: f64,
    y_frequency: f64,
    input_offset_x: f64,
    input_offset_y: f64,
    output_offset: f64,
}

impl AugmentedNoiseFn {
    pub fn new(
        noise_function_type: NoiseFnTypes,
        seed: u32,
        amplitude: f64,
        x_frequency: f64,
        y_frequency: f64,
        input_offset_x: f64,
        input_offset_y: f64,
        output_offset: f64,
    ) -> Self {
        let noise_function = match noise_function_type {
            NoiseFnTypes::OpenSimplex => Box::new(OpenSimplex::new(seed)),
        };

        return Self {
            noise_function,
            amplitude,
            x_frequency,
            y_frequency,
            input_offset_x,
            input_offset_y,
            output_offset,
        };
    }

    pub fn sample(&self, x: f64, y: f64) -> f64 {
        return self.amplitude
            * self.noise_function.get([
                x * self.x_frequency + self.input_offset_x,
                y * self.y_frequency + self.input_offset_y,
            ])
            + self.output_offset;
    }
}

impl Default for AugmentedNoiseFn {
    fn default() -> Self {
        return Self::new(
            NoiseFnTypes::OpenSimplex,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u32,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        );
    }
}

impl From<[f64; 6]> for AugmentedNoiseFn {
    fn from(item: [f64; 6]) -> Self {
        return Self::new(
            NoiseFnTypes::OpenSimplex,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u32,
            item[0],
            item[1],
            item[2],
            item[3],
            item[4],
            item[5],
        );
    }
}

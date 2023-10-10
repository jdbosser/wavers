//!
//! # Wavers
//! WaveRs is a fast and lightweight library for reading and writing ``wav`` files.
//! Currently, it supports reading and writing of ``i16``, ``i32``, ``f32``, and ``f64`` audio samples.
//!
//! ## Highlights
//! * Fast and lightweight
//! * Simple API, read a wav file with ``read`` and write a wav file with ``write``
//! * Easy and efficient conversion between different types of audio samples.
//! * Support for the ``ndarray`` crate.
//!
//! ## Crate Status
//! * This crate is currently in development. Changes to the core API will either not happen or they will be kept to a minimum. Any planned additions to the API will be built on top of the existing API.
//! * Documentation is currently in progress, it is mostly complete but will be updated as necessary.
//! * The API is tested, but there can always be more tests.
//! * The crate has been benchmarked, but there can always be more benchmarks.
//! * Some examples of planned features:
//!     * Support for reading and writing of ``i24`` audio samples.
//!     * Support iteration over samples in a wav file beyond calling ``iter()`` on the samples. Will providing windowing and other useful features.
//!     * Investigate the performance of the ``write`` function.
//!     * Channel wise iteration over samples in a wav file.
//!
//! ## Examples
//! The following examples show how to read and write a wav file, as well as retrieving information from the header.
//!
//!
//! ## Reading
//!
//! ```no_run
//! use wavers::{Wav, read};
//! use std::path::Path;
//!
//! fn main() {
//! 	let fp = "path/to/wav.wav";
//!     // creates a Wav file struct, does not read the audio data. Just the header information.
//!     let wav: Wav<i16> = Wav::from_path(fp).unwrap();
//!     // or to read the audio data directly
//!     let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(fp).unwrap();
//!     // samples can be derefed to a slice of samples
//!     let samples: &[i16] = &samples;
//! }
//! ```
//!
//! ## Conversion
//! ```no_run
//! use wavers::{Wav, read, ConvertTo};
//! use std::path::Path;
//!
//! fn main() {
//!     // Two ways of converted a wav file
//!     let fp: "./path/to/i16_encoded_wav.wav";
//!     let wav: Wav<f32> = Wav::from_path(fp).unwrap();
//!     // conversion happens automatically when you read
//!     let samples: &[f32] = &wav.read().unwrap();
//!
//!     // or read and then call the convert function on the samples.
//!     let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(fp).unwrap();
//!     let samples: &[f32] = &samples.convert();
//! }
//! ```
//!
//! ## Writing
//! ```no_run
//! use wavers::Wav;
//! use std::path::Path;
//!
//!
//! fn main() {
//!
//! 	let fp: &Path = &Path::new("path/to/wav.wav");
//! 	let out_fp: &Path = &Path::new("out/path/to/wav.wav");
//!
//!     // two main ways, read and write as the type when reading
//!     let wav: Wav<i16> = Wav::from_path(fp).unwrap();
//!     wav.write(out_fp).unwrap();
//!
//!     // or read, convert, and write
//!     let (samples, sample_rate): (Samples<i16>,i32) = read::<i16, _>(fp).unwrap();
//!     let sample_rate = wav.sample_rate();
//!     let n_channels = wav.n_channels();
//!
//!     let samples: &[f32] = &samples.convert();
//!     write(out_fp, samples, sample_rate, n_channels).unwrap();
//! }
//!
//! ```
//! ## Wav Utilities
//! ```no_run
//! use wavers::wav_spec;
//! fn main() {
//!	    let fp = "path/to/wav.wav";
//!     let wav: Wav<i16> = Wav::from_path(fp).unwrap();
//!     let sample_rate = wav.sample_rate();
//!     let n_channels = wav.n_channels();
//!     let duration = wav.duration();
//!     let encoding = wav.encoding();
//!     let (sample_rate, n_channels, duration, encoing) = wav_spec(fp).unwrap();
//! }
//! ```
//!
//! ## Features
//! The following section describes the features available in the WaveRs crate.
//! ### Ndarray
//!
//! The ``ndarray`` feature is used to provide functions that allow wav files to be read as ``ndarray`` 2-D arrays (samples x channels). There are two functions provided, ``into_ndarray`` and ``as_ndarray``. ``into_ndarray`` consumes the samples and ``as_ndarray`` creates a ``Array2`` from the samples.
//!
//! ```no_run
//! use wavers::{read, Wav, AsNdarray, IntoNdarray};
//! use ndarray::{Array2, CowArray2};
//!
//! fn main() {
//! 	let fp = "path/to/wav.wav";
//!     let wav: Wav<i16> = Wav::from_path(fp).unwrap();
//!
//!     // does not consume the wav file struct
//! 	let (i16_array, sample_rate): (Array2<i16>, i32) = wav.as_ndarray().unwrap();
//!     
//!    // consumes the wav file struct
//! 	let (i16_array, sample_rate): (Array2<i16>, i32) = wav.into_ndarray().unwrap();
//! }
//! ```
//!
//! ## Benchmarks
//! To check out the benchmarks head on over to the benchmarks wiki page on the WaveRs <a href=https://github.com/jmg049/wavers/wiki/Benchmarks>GitHub</a>.
//! Benchmarks were conducted on the reading and writing functionality of WaveRs and compared to the ``hound`` crate.
//!

mod conversion;
mod core;
mod error;
mod header;

use std::fs;
use std::io::Write;
use std::path::Path;

pub use crate::conversion::{AudioSample, ConvertTo};

pub use crate::conversion::ConvertSlice;

pub use crate::core::ReadSeek;
pub use crate::core::{wav_spec, Samples, Wav, WavType};
pub use crate::error::{WaversError, WaversResult};
use crate::header::DATA;
pub use crate::header::{FmtChunk, WavHeader};

/// Reads a wav file and returns the samples and the sample rate.
///
/// Throws an error if the file cannot be opened.
///
/// # Examples
///
/// ```no_run
/// use wavers::{read, Wav, Samples};
///
/// fn main() {
///     let fp = "path/to/wav.wav";
///     // reads the audio data as i16 samples
///     let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(fp).unwrap();
///     // or read the same file as f32 samples
///     let (samples, sample_rate): (Samples<f32>, i32) = read::<f32, _>(fp).unwrap();
/// }
///
#[inline(always)]
pub fn read<T: AudioSample, P: AsRef<Path>>(path: P) -> WaversResult<(Samples<T>, i32)>
where
    i16: ConvertTo<T>,
    i32: ConvertTo<T>,
    f32: ConvertTo<T>,
    f64: ConvertTo<T>,
    Box<[i16]>: ConvertSlice<T>,
    Box<[i32]>: ConvertSlice<T>,
    Box<[f32]>: ConvertSlice<T>,
    Box<[f64]>: ConvertSlice<T>,
{
    let mut wav: Wav<T> = Wav::from_path(path)?;
    let samples = wav.read()?;
    Ok((samples, wav.sample_rate()))
}

/// Writes wav samples to disk.
///
/// # Examples
///
/// The code below will generate a wav file from a 10 second, 1-channel sine wave and write it to disk.
/// ```
/// use wavers::{read,write, Samples, AudioSample, ConvertTo, ConvertSlice};
///
/// fn main() {
///     let fp = "./wav.wav";
///     let sr: i32 = 16000;
///     let duration = 10;
///     let mut samples: Vec<f32> = (0..sr * duration).map(|x| (x as f32 / sr as f32)).collect();
///     for sample in samples.iter_mut() {
///         *sample *= 440.0 * 2.0 * std::f32::consts::PI;
///         *sample = sample.sin();
///         *sample *= i16::MAX as f32;
///     }
///     let samples: Samples<f32> = Samples::from(samples.into_boxed_slice()).convert();
///     assert!(write(fp, &samples, sr, 1).is_ok());
///     std::fs::remove_file(fp).unwrap();
/// }
///
#[inline(always)]
pub fn write<T: AudioSample, P: AsRef<Path>>(
    fp: P,
    samples: &[T],
    sample_rate: i32,
    n_channels: u16,
) -> WaversResult<()>
where
    i16: ConvertTo<T>,
    i32: ConvertTo<T>,
    f32: ConvertTo<T>,
    f64: ConvertTo<T>,
    Box<[i16]>: ConvertSlice<T>,
    Box<[i32]>: ConvertSlice<T>,
    Box<[f32]>: ConvertSlice<T>,
    Box<[f64]>: ConvertSlice<T>,
{
    let s = Samples::from(samples);
    let samples_bytes = s.as_bytes();

    let new_header = WavHeader::new_header::<T>(sample_rate, n_channels, s.len())?;
    let header_bytes = new_header.as_bytes();
    let mut f = fs::File::create(fp)?;
    f.write_all(&header_bytes)?;
    f.write_all(&DATA)?;
    let data_size_bytes = samples_bytes.len() as u32; // write up to the data size
    f.write_all(&data_size_bytes.to_ne_bytes())?; // write the data size
    f.write_all(&samples_bytes)?; // write the data
    Ok(())
}

#[cfg(test)]
mod tests {
    use approx_eq::assert_approx_eq;
    use std::io::BufRead;
    use std::{fs::File, path::Path, str::FromStr};

    use super::{read, write, Samples, Wav};

    const TEST_OUTPUT: &str = "./test_resources/tmp/";

    #[test]
    fn test_write() {
        let expected_path = "./test_resources/one_channel_f32.txt";
        let out_path = "./test_resources/tmp/one_channel_f32_tmp.wav";

        let mut wav: Wav<f32> = Wav::from_path("./test_resources/one_channel_i16.wav")
            .expect("Failed to open file wav file");

        let samples = wav.read().expect("Failed to read data");

        write(out_path, &samples, wav.sample_rate(), wav.n_channels())
            .expect("Failed to write data");

        let mut wav: Wav<f32> = Wav::from_path(out_path).expect("Failed to open file wav file");
        let samples: Samples<f32> = wav.read().expect("Failed to read data");

        let expected: Vec<f32> = read_text_to_vec(expected_path).expect("failed to load from txt");

        for (exp, act) in expected.iter().zip(samples.as_ref()) {
            assert_approx_eq!(*exp as f64, *act as f64, 1e-4);
        }
        std::fs::remove_file(Path::new(&out_path)).unwrap();
    }

    #[test]
    fn test_read() {
        let input_path = "./test_resources/one_channel_i16.wav";
        let expected_path = "./test_resources/one_channel_i16.txt";
        let expected_sr = 16000;

        let mut wav: Wav<i16> = Wav::from_path(input_path).expect("Failed to open file wav file");
        let samples: Samples<i16> = wav.read().expect("Failed to read data");
        let actual_sr = wav.sample_rate();
        let expected: Vec<i16> = read_text_to_vec(expected_path).expect("failed to load from txt");

        assert_eq!(expected_sr, actual_sr, "Sample rates do not match");
        for (exp, act) in expected.iter().zip(samples.as_ref()) {
            assert_eq!(*exp, *act, "Samples do not match");
        }
    }

    use std::stringify;
    macro_rules! read_tests {
        ($($T:ident), *) => {
            $(
                paste::item! {
                    #[test]
                    fn [<read_$T>]() {
                        let t_string: &str = stringify!($T);

                        let wav_str = format!("./test_resources/one_channel_{}.wav", t_string);
                        let expected_str = format!("./test_resources/one_channel_{}.txt", t_string);

                        let (sample_data, _): (Samples<$T>, i32) = match read::<$T, _>(&wav_str) {
                            Ok((s, sr)) => (s, sr),
                            Err(e) => {eprintln!("{}\n{}", wav_str, e); panic!("Failed to read wav file")}
                        };

                        let expected_data: Vec<$T> = match read_text_to_vec(Path::new(&expected_str)) {
                            Ok(w) => w,
                            Err(e) => {eprintln!("{}\n{}", wav_str, e); panic!("Failed to read txt file")}
                        };

                        for (expected, actual) in expected_data.iter().zip(sample_data.iter()) {
                            assert_eq!(*expected, *actual, "{} != {}", expected, actual);
                        }
                    }
                }
            )*
        }
    }
    read_tests!(i16, i32, f32, f64);

    macro_rules! write_tests {
        ($($T:ident), *) => {
            $(
                paste::item! {
                    #[test]
                    fn [<write_$T>]() {
                        if !Path::new(TEST_OUTPUT).exists() {
                            std::fs::create_dir(TEST_OUTPUT).unwrap();
                        }
                        let t_string: &str = stringify!($T);

                        let wav_str = format!("./test_resources/one_channel_{}.wav", t_string);
                        let expected_str = format!("./test_resources/one_channel_{}.txt", t_string);

                        let mut wav: Wav<$T> =
                            Wav::from_path(wav_str).expect("Failed to create wav file");
                        let expected_samples: Samples<$T> = Samples::from(
                            read_text_to_vec(&Path::new(&expected_str)).expect("Failed to read to vec"),
                        );


                        let out = format!("{}_one_channel_{}.wav", TEST_OUTPUT, t_string);
                        let out_path = Path::new(&out);

                        wav.write::<$T, _>(out_path)
                            .expect("Failed to write file");

                        let mut new_wav: Wav<$T> = Wav::<$T>::from_path(out_path).unwrap();

                        for (expected, actual) in expected_samples
                            .iter()
                            .zip(new_wav.read().unwrap().iter())
                        {
                            assert_eq!(expected, actual, "{} != {}", expected, actual);
                        }
                        std::fs::remove_file(Path::new(&out_path)).unwrap();
                    }
                }
            )*
        };
    }

    write_tests!(i16, i32, f32, f64);

    use crate::ConvertSlice;
    #[test]
    fn write_sin_wav() {
        let fp = "./wav.wav";
        let sr: i32 = 16000;
        let duration = 10;
        let mut samples: Vec<f32> = (0..sr * duration).map(|x| (x as f32 / sr as f32)).collect();
        for sample in samples.iter_mut() {
            *sample *= 440.0 * 2.0 * std::f32::consts::PI;
            *sample = sample.sin();
            *sample *= i16::MAX as f32;
        }
        let samples: Samples<f32> = Samples::from(samples.into_boxed_slice().convert_slice());

        write(fp, &samples, sr, 1).unwrap();
        std::fs::remove_file(fp).unwrap();
    }

    fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(std::io::BufReader::new(file).lines())
    }

    fn read_text_to_vec<T: FromStr, P: AsRef<Path>>(
        fp: P,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        let mut data = Vec::new();
        let lines = read_lines(fp)?;
        for line in lines {
            let line = line?;
            for sample in line.split(" ") {
                let parsed_sample: T = match sample.trim().parse::<T>() {
                    Ok(num) => num,
                    Err(err) => {
                        eprintln!("Failed to parse {}", sample);
                        panic!("{}", err)
                    }
                };
                data.push(parsed_sample);
            }
        }
        Ok(data)
    }
}

#[cfg(feature = "ndarray")]
pub use conversion::{AsNdarray, IntoNdarray};

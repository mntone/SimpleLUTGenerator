use std::fs::File;
use std::io::Write;
use std::str;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
}

impl Default for Color {
  fn default() -> Self {
    Color {
      red:   0_f64,
      green: 0_f64,
      blue:  0_f64,
    }
  }
}

const LUT_SIZE: usize = 256;
const NUM16_8BIT_F64: f64 = 16.0 / 255.0;
const NUM219_8BIT_F64: f64 = 219.0 / 255.0;
const NUM235_8BIT_F64: f64 = 235.0 / 255.0;

fn remove_srgb(x: f64) -> f64 {
  if x < 0.04045_f64 {
    x / 12.92
  } else {
    ((x + 0.055) / 1.055).powf(2.4)
  }
}

fn add_bt709(x: f64) -> f64 {
  if x < 0.018_f64 {
    4.5 * x
  } else {
    1.099 * x.powf(0.45) - 0.099
  }
}

fn cvt_full_8bit(x: f64) -> f64 {
  if x <= NUM16_8BIT_F64 {
    0.0
  } else if x >= NUM235_8BIT_F64 {
    1.0
  } else {
    (255.0 / 219.0) * (x - NUM16_8BIT_F64)
  }
}

fn cvt_limited_8bit(x: f64) -> f64 {
  NUM219_8BIT_F64 * x + NUM16_8BIT_F64
}

fn generate_lut<F>(func: F) -> [Color; LUT_SIZE] where F: Fn(f64) -> f64 {
  let mut lut: [Color; LUT_SIZE] = [Default::default(); LUT_SIZE];
  for (i, val) in lut.iter_mut().enumerate() {
    let base: f64 = (i as f64) / ((LUT_SIZE - 1) as f64);
    let result: f64 = func(base);
    val.red  = result;
    val.green = result;
    val.blue  = result;
  }
  lut
}

fn save_lut<F>(filename: &str, title: &str, func: F) -> Result<(), Box<dyn std::error::Error>> where F: Fn(f64) -> f64 {
  let mut file = File::create(format!("./output/{}", filename))?;
  let lut = generate_lut(func);
  write!(file, "TITLE {}\n", title)?;
  write!(file, "LUT_1D_SIZE {}\n", LUT_SIZE)?;
  for data in lut.iter() {
    write!(file, "{} {} {}\n", data.red, data.green, data.blue)?;
  }
  file.flush()?;
  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let conv = |x| add_bt709(remove_srgb(x));
  save_lut(
    "sRGB_BT709_gammacorr_8bit.cube",
    "sRGB to BT.709 gamma correction (8-bit)",
    conv)?;

  save_lut(
    "limited_to_full_8bit.cube",
    "RGB limited range to full range (8-bit)",
    cvt_full_8bit)?;

  save_lut(
    "full_to_limited_8bit.cube",
    "RGB full range to limited range (8-bit)",
    cvt_limited_8bit)?;
  Ok(())
}

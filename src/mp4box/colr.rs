use serde::Serialize;

use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ColrBox {
    pub color_parameter_type: u32,
    pub color_primaries: u16,
    pub transfer_characteristics: u16,
    pub matrix_coefficients: u16,
}

#[allow(dead_code)]
enum ColorParameterType {
    Nclc = 0x6e636c63,
    Prof = 0x70727066,
    Unknown = 0x00000000,
}

impl Default for ColrBox {
    fn default() -> Self {
        ColrBox {
            // color_parameter_type: u32::from_be_bytes([b'n', b'c', b'l', b'c']),
            color_parameter_type: ColorParameterType::Nclc as u32,
            color_primaries: 1,
            transfer_characteristics: 1,
            matrix_coefficients: 1,
        }
    }
}

impl ColrBox {
    pub fn new() -> Self {
        ColrBox {
            color_parameter_type: ColorParameterType::Nclc as u32,
            color_primaries: 1,
            transfer_characteristics: 1,
            matrix_coefficients: 1,
        }
    }

    pub fn get_type(&self) -> BoxType {
        BoxType::ColrBox
    }

    pub fn get_size(&self) -> u64 {
        // todo: be more descriptive here. use magic number for now.
        18
    }
}

impl Mp4Box for ColrBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "colr_parameter_type={} color_primaries={} transfer_characteristics={} matrix_coeffients={}",
            self.color_parameter_type,
            self.color_primaries,
            self.transfer_characteristics,
            self.matrix_coefficients
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for ColrBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start = box_start(reader)?;
        let size = reader.read_u32::<BigEndian>()?;
        let colr_type = reader.read_u32::<BigEndian>()?;
        let color_parameter_type = reader.read_u32::<BigEndian>()?;

        let color_primaries = reader.read_u16::<BigEndian>()?;
        let transfer_characteristics = reader.read_u16::<BigEndian>()?;
        let matrix_coefficients = reader.read_u16::<BigEndian>()?;

        Ok(ColrBox {
            color_parameter_type,
            color_primaries,
            transfer_characteristics,
            matrix_coefficients,
        })
    }
}

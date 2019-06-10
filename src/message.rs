// Automatically generated rust module for 'message.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SerialMessage {
    pub word_1: u64,
    pub word_2: u64,
    pub word_3: u64,
}

impl<'a> MessageRead<'a> for SerialMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.word_1 = r.read_uint64(bytes)?,
                Ok(16) => msg.word_2 = r.read_uint64(bytes)?,
                Ok(24) => msg.word_3 = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SerialMessage {
    fn get_size(&self) -> usize {
        0
        + if self.word_1 == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.word_1) as u64) }
        + if self.word_2 == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.word_2) as u64) }
        + if self.word_3 == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.word_3) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.word_1 != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.word_1))?; }
        if self.word_2 != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.word_2))?; }
        if self.word_3 != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.word_3))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IdsMessage {
    pub id_0: i32,
    pub id_1: i32,
    pub id_2: i32,
    pub id_3: i32,
    pub id_4: i32,
    pub id_5: i32,
    pub id_6: i32,
    pub id_7: i32,
    pub id_8: i32,
    pub id_9: i32,
    pub id_10: i32,
    pub id_11: i32,
    pub id_12: i32,
    pub id_13: i32,
    pub id_14: i32,
    pub id_15: i32,
    pub id_16: i32,
    pub id_17: i32,
    pub id_18: i32,
    pub id_19: i32,
    pub id_20: i32,
    pub id_21: i32,
    pub id_22: i32,
    pub id_23: i32,
    pub id_24: i32,
    pub id_25: i32,
    pub id_26: i32,
    pub id_27: i32,
    pub id_28: i32,
    pub id_29: i32,
    pub id_30: i32,
    pub id_31: i32,
    pub id_32: i32,
    pub id_33: i32,
    pub id_34: i32,
    pub id_35: i32,
    pub id_36: i32,
    pub id_37: i32,
    pub id_38: i32,
    pub id_39: i32,
    pub id_40: i32,
    pub id_41: i32,
    pub id_42: i32,
    pub id_43: i32,
    pub id_44: i32,
    pub id_45: i32,
    pub id_46: i32,
    pub id_47: i32,
    pub id_48: i32,
    pub id_49: i32,
}

impl<'a> MessageRead<'a> for IdsMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id_0 = r.read_int32(bytes)?,
                Ok(16) => msg.id_1 = r.read_int32(bytes)?,
                Ok(24) => msg.id_2 = r.read_int32(bytes)?,
                Ok(32) => msg.id_3 = r.read_int32(bytes)?,
                Ok(40) => msg.id_4 = r.read_int32(bytes)?,
                Ok(48) => msg.id_5 = r.read_int32(bytes)?,
                Ok(56) => msg.id_6 = r.read_int32(bytes)?,
                Ok(64) => msg.id_7 = r.read_int32(bytes)?,
                Ok(72) => msg.id_8 = r.read_int32(bytes)?,
                Ok(80) => msg.id_9 = r.read_int32(bytes)?,
                Ok(88) => msg.id_10 = r.read_int32(bytes)?,
                Ok(96) => msg.id_11 = r.read_int32(bytes)?,
                Ok(104) => msg.id_12 = r.read_int32(bytes)?,
                Ok(112) => msg.id_13 = r.read_int32(bytes)?,
                Ok(120) => msg.id_14 = r.read_int32(bytes)?,
                Ok(128) => msg.id_15 = r.read_int32(bytes)?,
                Ok(136) => msg.id_16 = r.read_int32(bytes)?,
                Ok(144) => msg.id_17 = r.read_int32(bytes)?,
                Ok(152) => msg.id_18 = r.read_int32(bytes)?,
                Ok(160) => msg.id_19 = r.read_int32(bytes)?,
                Ok(168) => msg.id_20 = r.read_int32(bytes)?,
                Ok(176) => msg.id_21 = r.read_int32(bytes)?,
                Ok(184) => msg.id_22 = r.read_int32(bytes)?,
                Ok(192) => msg.id_23 = r.read_int32(bytes)?,
                Ok(200) => msg.id_24 = r.read_int32(bytes)?,
                Ok(208) => msg.id_25 = r.read_int32(bytes)?,
                Ok(216) => msg.id_26 = r.read_int32(bytes)?,
                Ok(224) => msg.id_27 = r.read_int32(bytes)?,
                Ok(232) => msg.id_28 = r.read_int32(bytes)?,
                Ok(240) => msg.id_29 = r.read_int32(bytes)?,
                Ok(248) => msg.id_30 = r.read_int32(bytes)?,
                Ok(256) => msg.id_31 = r.read_int32(bytes)?,
                Ok(264) => msg.id_32 = r.read_int32(bytes)?,
                Ok(272) => msg.id_33 = r.read_int32(bytes)?,
                Ok(280) => msg.id_34 = r.read_int32(bytes)?,
                Ok(288) => msg.id_35 = r.read_int32(bytes)?,
                Ok(296) => msg.id_36 = r.read_int32(bytes)?,
                Ok(304) => msg.id_37 = r.read_int32(bytes)?,
                Ok(312) => msg.id_38 = r.read_int32(bytes)?,
                Ok(320) => msg.id_39 = r.read_int32(bytes)?,
                Ok(328) => msg.id_40 = r.read_int32(bytes)?,
                Ok(336) => msg.id_41 = r.read_int32(bytes)?,
                Ok(344) => msg.id_42 = r.read_int32(bytes)?,
                Ok(352) => msg.id_43 = r.read_int32(bytes)?,
                Ok(360) => msg.id_44 = r.read_int32(bytes)?,
                Ok(368) => msg.id_45 = r.read_int32(bytes)?,
                Ok(376) => msg.id_46 = r.read_int32(bytes)?,
                Ok(384) => msg.id_47 = r.read_int32(bytes)?,
                Ok(392) => msg.id_48 = r.read_int32(bytes)?,
                Ok(400) => msg.id_49 = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for IdsMessage {
    fn get_size(&self) -> usize {
        0
        + if self.id_0 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_0) as u64) }
        + if self.id_1 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_1) as u64) }
        + if self.id_2 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_2) as u64) }
        + if self.id_3 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_3) as u64) }
        + if self.id_4 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_4) as u64) }
        + if self.id_5 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_5) as u64) }
        + if self.id_6 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_6) as u64) }
        + if self.id_7 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_7) as u64) }
        + if self.id_8 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_8) as u64) }
        + if self.id_9 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_9) as u64) }
        + if self.id_10 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_10) as u64) }
        + if self.id_11 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_11) as u64) }
        + if self.id_12 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_12) as u64) }
        + if self.id_13 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_13) as u64) }
        + if self.id_14 == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.id_14) as u64) }
        + if self.id_15 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_15) as u64) }
        + if self.id_16 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_16) as u64) }
        + if self.id_17 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_17) as u64) }
        + if self.id_18 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_18) as u64) }
        + if self.id_19 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_19) as u64) }
        + if self.id_20 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_20) as u64) }
        + if self.id_21 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_21) as u64) }
        + if self.id_22 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_22) as u64) }
        + if self.id_23 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_23) as u64) }
        + if self.id_24 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_24) as u64) }
        + if self.id_25 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_25) as u64) }
        + if self.id_26 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_26) as u64) }
        + if self.id_27 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_27) as u64) }
        + if self.id_28 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_28) as u64) }
        + if self.id_29 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_29) as u64) }
        + if self.id_30 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_30) as u64) }
        + if self.id_31 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_31) as u64) }
        + if self.id_32 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_32) as u64) }
        + if self.id_33 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_33) as u64) }
        + if self.id_34 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_34) as u64) }
        + if self.id_35 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_35) as u64) }
        + if self.id_36 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_36) as u64) }
        + if self.id_37 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_37) as u64) }
        + if self.id_38 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_38) as u64) }
        + if self.id_39 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_39) as u64) }
        + if self.id_40 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_40) as u64) }
        + if self.id_41 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_41) as u64) }
        + if self.id_42 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_42) as u64) }
        + if self.id_43 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_43) as u64) }
        + if self.id_44 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_44) as u64) }
        + if self.id_45 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_45) as u64) }
        + if self.id_46 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_46) as u64) }
        + if self.id_47 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_47) as u64) }
        + if self.id_48 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_48) as u64) }
        + if self.id_49 == 0i32 { 0 } else { 2 + sizeof_varint(*(&self.id_49) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id_0 != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.id_0))?; }
        if self.id_1 != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.id_1))?; }
        if self.id_2 != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.id_2))?; }
        if self.id_3 != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.id_3))?; }
        if self.id_4 != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.id_4))?; }
        if self.id_5 != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.id_5))?; }
        if self.id_6 != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.id_6))?; }
        if self.id_7 != 0i32 { w.write_with_tag(64, |w| w.write_int32(*&self.id_7))?; }
        if self.id_8 != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.id_8))?; }
        if self.id_9 != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.id_9))?; }
        if self.id_10 != 0i32 { w.write_with_tag(88, |w| w.write_int32(*&self.id_10))?; }
        if self.id_11 != 0i32 { w.write_with_tag(96, |w| w.write_int32(*&self.id_11))?; }
        if self.id_12 != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.id_12))?; }
        if self.id_13 != 0i32 { w.write_with_tag(112, |w| w.write_int32(*&self.id_13))?; }
        if self.id_14 != 0i32 { w.write_with_tag(120, |w| w.write_int32(*&self.id_14))?; }
        if self.id_15 != 0i32 { w.write_with_tag(128, |w| w.write_int32(*&self.id_15))?; }
        if self.id_16 != 0i32 { w.write_with_tag(136, |w| w.write_int32(*&self.id_16))?; }
        if self.id_17 != 0i32 { w.write_with_tag(144, |w| w.write_int32(*&self.id_17))?; }
        if self.id_18 != 0i32 { w.write_with_tag(152, |w| w.write_int32(*&self.id_18))?; }
        if self.id_19 != 0i32 { w.write_with_tag(160, |w| w.write_int32(*&self.id_19))?; }
        if self.id_20 != 0i32 { w.write_with_tag(168, |w| w.write_int32(*&self.id_20))?; }
        if self.id_21 != 0i32 { w.write_with_tag(176, |w| w.write_int32(*&self.id_21))?; }
        if self.id_22 != 0i32 { w.write_with_tag(184, |w| w.write_int32(*&self.id_22))?; }
        if self.id_23 != 0i32 { w.write_with_tag(192, |w| w.write_int32(*&self.id_23))?; }
        if self.id_24 != 0i32 { w.write_with_tag(200, |w| w.write_int32(*&self.id_24))?; }
        if self.id_25 != 0i32 { w.write_with_tag(208, |w| w.write_int32(*&self.id_25))?; }
        if self.id_26 != 0i32 { w.write_with_tag(216, |w| w.write_int32(*&self.id_26))?; }
        if self.id_27 != 0i32 { w.write_with_tag(224, |w| w.write_int32(*&self.id_27))?; }
        if self.id_28 != 0i32 { w.write_with_tag(232, |w| w.write_int32(*&self.id_28))?; }
        if self.id_29 != 0i32 { w.write_with_tag(240, |w| w.write_int32(*&self.id_29))?; }
        if self.id_30 != 0i32 { w.write_with_tag(248, |w| w.write_int32(*&self.id_30))?; }
        if self.id_31 != 0i32 { w.write_with_tag(256, |w| w.write_int32(*&self.id_31))?; }
        if self.id_32 != 0i32 { w.write_with_tag(264, |w| w.write_int32(*&self.id_32))?; }
        if self.id_33 != 0i32 { w.write_with_tag(272, |w| w.write_int32(*&self.id_33))?; }
        if self.id_34 != 0i32 { w.write_with_tag(280, |w| w.write_int32(*&self.id_34))?; }
        if self.id_35 != 0i32 { w.write_with_tag(288, |w| w.write_int32(*&self.id_35))?; }
        if self.id_36 != 0i32 { w.write_with_tag(296, |w| w.write_int32(*&self.id_36))?; }
        if self.id_37 != 0i32 { w.write_with_tag(304, |w| w.write_int32(*&self.id_37))?; }
        if self.id_38 != 0i32 { w.write_with_tag(312, |w| w.write_int32(*&self.id_38))?; }
        if self.id_39 != 0i32 { w.write_with_tag(320, |w| w.write_int32(*&self.id_39))?; }
        if self.id_40 != 0i32 { w.write_with_tag(328, |w| w.write_int32(*&self.id_40))?; }
        if self.id_41 != 0i32 { w.write_with_tag(336, |w| w.write_int32(*&self.id_41))?; }
        if self.id_42 != 0i32 { w.write_with_tag(344, |w| w.write_int32(*&self.id_42))?; }
        if self.id_43 != 0i32 { w.write_with_tag(352, |w| w.write_int32(*&self.id_43))?; }
        if self.id_44 != 0i32 { w.write_with_tag(360, |w| w.write_int32(*&self.id_44))?; }
        if self.id_45 != 0i32 { w.write_with_tag(368, |w| w.write_int32(*&self.id_45))?; }
        if self.id_46 != 0i32 { w.write_with_tag(376, |w| w.write_int32(*&self.id_46))?; }
        if self.id_47 != 0i32 { w.write_with_tag(384, |w| w.write_int32(*&self.id_47))?; }
        if self.id_48 != 0i32 { w.write_with_tag(392, |w| w.write_int32(*&self.id_48))?; }
        if self.id_49 != 0i32 { w.write_with_tag(400, |w| w.write_int32(*&self.id_49))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SerialsMessage {
    pub serial_0: Option<message::SerialMessage>,
    pub serial_1: Option<message::SerialMessage>,
    pub serial_2: Option<message::SerialMessage>,
    pub serial_3: Option<message::SerialMessage>,
    pub serial_4: Option<message::SerialMessage>,
    pub serial_5: Option<message::SerialMessage>,
    pub serial_6: Option<message::SerialMessage>,
    pub serial_7: Option<message::SerialMessage>,
    pub serial_8: Option<message::SerialMessage>,
    pub serial_9: Option<message::SerialMessage>,
    pub serial_10: Option<message::SerialMessage>,
    pub serial_11: Option<message::SerialMessage>,
    pub serial_12: Option<message::SerialMessage>,
    pub serial_13: Option<message::SerialMessage>,
    pub serial_14: Option<message::SerialMessage>,
    pub serial_15: Option<message::SerialMessage>,
    pub serial_16: Option<message::SerialMessage>,
    pub serial_17: Option<message::SerialMessage>,
    pub serial_18: Option<message::SerialMessage>,
    pub serial_19: Option<message::SerialMessage>,
    pub serial_20: Option<message::SerialMessage>,
    pub serial_21: Option<message::SerialMessage>,
    pub serial_22: Option<message::SerialMessage>,
    pub serial_23: Option<message::SerialMessage>,
    pub serial_24: Option<message::SerialMessage>,
    pub serial_25: Option<message::SerialMessage>,
    pub serial_26: Option<message::SerialMessage>,
    pub serial_27: Option<message::SerialMessage>,
    pub serial_28: Option<message::SerialMessage>,
    pub serial_29: Option<message::SerialMessage>,
    pub serial_30: Option<message::SerialMessage>,
    pub serial_31: Option<message::SerialMessage>,
    pub serial_32: Option<message::SerialMessage>,
    pub serial_33: Option<message::SerialMessage>,
    pub serial_34: Option<message::SerialMessage>,
    pub serial_35: Option<message::SerialMessage>,
    pub serial_36: Option<message::SerialMessage>,
    pub serial_37: Option<message::SerialMessage>,
    pub serial_38: Option<message::SerialMessage>,
    pub serial_39: Option<message::SerialMessage>,
    pub serial_40: Option<message::SerialMessage>,
    pub serial_41: Option<message::SerialMessage>,
    pub serial_42: Option<message::SerialMessage>,
    pub serial_43: Option<message::SerialMessage>,
    pub serial_44: Option<message::SerialMessage>,
    pub serial_45: Option<message::SerialMessage>,
    pub serial_46: Option<message::SerialMessage>,
    pub serial_47: Option<message::SerialMessage>,
    pub serial_48: Option<message::SerialMessage>,
    pub serial_49: Option<message::SerialMessage>,
}

impl<'a> MessageRead<'a> for SerialsMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.serial_0 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(18) => msg.serial_1 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(26) => msg.serial_2 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(34) => msg.serial_3 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(42) => msg.serial_4 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(50) => msg.serial_5 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(58) => msg.serial_6 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(66) => msg.serial_7 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(74) => msg.serial_8 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(82) => msg.serial_9 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(90) => msg.serial_10 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(98) => msg.serial_11 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(106) => msg.serial_12 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(114) => msg.serial_13 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(122) => msg.serial_14 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(130) => msg.serial_15 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(138) => msg.serial_16 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(146) => msg.serial_17 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(154) => msg.serial_18 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(162) => msg.serial_19 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(170) => msg.serial_20 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(178) => msg.serial_21 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(186) => msg.serial_22 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(194) => msg.serial_23 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(202) => msg.serial_24 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(210) => msg.serial_25 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(218) => msg.serial_26 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(226) => msg.serial_27 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(234) => msg.serial_28 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(242) => msg.serial_29 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(250) => msg.serial_30 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(258) => msg.serial_31 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(266) => msg.serial_32 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(274) => msg.serial_33 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(282) => msg.serial_34 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(290) => msg.serial_35 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(298) => msg.serial_36 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(306) => msg.serial_37 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(314) => msg.serial_38 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(322) => msg.serial_39 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(330) => msg.serial_40 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(338) => msg.serial_41 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(346) => msg.serial_42 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(354) => msg.serial_43 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(362) => msg.serial_44 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(370) => msg.serial_45 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(378) => msg.serial_46 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(386) => msg.serial_47 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(394) => msg.serial_48 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(402) => msg.serial_49 = Some(r.read_message::<message::SerialMessage>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SerialsMessage {
    fn get_size(&self) -> usize {
        0
        + self.serial_0.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_1.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_2.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_3.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_4.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_5.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_6.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_7.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_8.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_9.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_10.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_11.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_12.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_13.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_14.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serial_15.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_16.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_17.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_18.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_19.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_20.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_21.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_22.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_23.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_24.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_25.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_26.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_27.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_28.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_29.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_30.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_31.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_32.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_33.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_34.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_35.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_36.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_37.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_38.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_39.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_40.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_41.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_42.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_43.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_44.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_45.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_46.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_47.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_48.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.serial_49.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.serial_0 { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_1 { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_2 { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_3 { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_4 { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_5 { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_6 { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_7 { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_8 { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_9 { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_10 { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_11 { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_12 { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_13 { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_14 { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_15 { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_16 { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_17 { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_18 { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_19 { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_20 { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_21 { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_22 { w.write_with_tag(186, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_23 { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_24 { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_25 { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_26 { w.write_with_tag(218, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_27 { w.write_with_tag(226, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_28 { w.write_with_tag(234, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_29 { w.write_with_tag(242, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_30 { w.write_with_tag(250, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_31 { w.write_with_tag(258, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_32 { w.write_with_tag(266, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_33 { w.write_with_tag(274, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_34 { w.write_with_tag(282, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_35 { w.write_with_tag(290, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_36 { w.write_with_tag(298, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_37 { w.write_with_tag(306, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_38 { w.write_with_tag(314, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_39 { w.write_with_tag(322, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_40 { w.write_with_tag(330, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_41 { w.write_with_tag(338, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_42 { w.write_with_tag(346, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_43 { w.write_with_tag(354, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_44 { w.write_with_tag(362, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_45 { w.write_with_tag(370, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_46 { w.write_with_tag(378, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_47 { w.write_with_tag(386, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_48 { w.write_with_tag(394, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serial_49 { w.write_with_tag(402, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelMessage {
    pub ids: Option<message::IdsMessage>,
    pub serials: Option<message::SerialsMessage>,
}

impl<'a> MessageRead<'a> for ChannelMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids = Some(r.read_message::<message::IdsMessage>(bytes)?),
                Ok(18) => msg.serials = Some(r.read_message::<message::SerialsMessage>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelMessage {
    fn get_size(&self) -> usize {
        0
        + self.ids.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.serials.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ids { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.serials { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}


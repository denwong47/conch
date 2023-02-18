use std::{default, fmt};
use strum_macros::EnumIter;

use enum_index::*;

use conch_base_models::{ANSIEscapeCode, IntoANSIEscapeCode, ModifierError};
use conch_macros::ansi_enum_builder as builder;

type OptionU8 = Option<u8>;

macro_rules! color_builder {
    (
        $enum_name:ident,
        $apply_idx:literal,
        $reset_idx:literal
    ) => {
        /// An Enum for ANSI 256-colour codes.
        #[derive(Debug, EnumIter, EnumIndex, PartialEq)]
        #[index_type(OptionU8)]
        pub enum $enum_name {
            #[index(Some(0))]
            Black,

            #[index(Some(1))]
            Red,

            #[index(Some(2))]
            Green,

            #[index(Some(3))]
            Yellow,

            #[index(Some(4))]
            Blue,

            #[index(Some(5))]
            Magenta,

            #[index(Some(6))]
            Cyan,

            #[index(Some(7))]
            White,

            #[index(Some(8))]
            BrightBlack,

            #[index(Some(9))]
            BrightRed,

            #[index(Some(10))]
            BrightGreen,

            #[index(Some(11))]
            BrightYellow,

            #[index(Some(12))]
            BrightBlue,

            #[index(Some(13))]
            BrightMagenta,

            #[index(Some(14))]
            BrightCyan,

            #[index(Some(15))]
            BrightWhite,

            #[index(Some(16))]
            R0G0B0,

            #[index(Some(17))]
            R0G0B1,

            #[index(Some(18))]
            R0G0B2,

            #[index(Some(19))]
            R0G0B3,

            #[index(Some(20))]
            R0G0B4,

            #[index(Some(21))]
            R0G0B5,

            #[index(Some(22))]
            R0G1B0,

            #[index(Some(23))]
            R0G1B1,

            #[index(Some(24))]
            R0G1B2,

            #[index(Some(25))]
            R0G1B3,

            #[index(Some(26))]
            R0G1B4,

            #[index(Some(27))]
            R0G1B5,

            #[index(Some(28))]
            R0G2B0,

            #[index(Some(29))]
            R0G2B1,

            #[index(Some(30))]
            R0G2B2,

            #[index(Some(31))]
            R0G2B3,

            #[index(Some(32))]
            R0G2B4,

            #[index(Some(33))]
            R0G2B5,

            #[index(Some(34))]
            R0G3B0,

            #[index(Some(35))]
            R0G3B1,

            #[index(Some(36))]
            R0G3B2,

            #[index(Some(37))]
            R0G3B3,

            #[index(Some(38))]
            R0G3B4,

            #[index(Some(39))]
            R0G3B5,

            #[index(Some(40))]
            R0G4B0,

            #[index(Some(41))]
            R0G4B1,

            #[index(Some(42))]
            R0G4B2,

            #[index(Some(43))]
            R0G4B3,

            #[index(Some(44))]
            R0G4B4,

            #[index(Some(45))]
            R0G4B5,

            #[index(Some(46))]
            R0G5B0,

            #[index(Some(47))]
            R0G5B1,

            #[index(Some(48))]
            R0G5B2,

            #[index(Some(49))]
            R0G5B3,

            #[index(Some(50))]
            R0G5B4,

            #[index(Some(51))]
            R0G5B5,

            #[index(Some(52))]
            R1G0B0,

            #[index(Some(53))]
            R1G0B1,

            #[index(Some(54))]
            R1G0B2,

            #[index(Some(55))]
            R1G0B3,

            #[index(Some(56))]
            R1G0B4,

            #[index(Some(57))]
            R1G0B5,

            #[index(Some(58))]
            R1G1B0,

            #[index(Some(59))]
            R1G1B1,

            #[index(Some(60))]
            R1G1B2,

            #[index(Some(61))]
            R1G1B3,

            #[index(Some(62))]
            R1G1B4,

            #[index(Some(63))]
            R1G1B5,

            #[index(Some(64))]
            R1G2B0,

            #[index(Some(65))]
            R1G2B1,

            #[index(Some(66))]
            R1G2B2,

            #[index(Some(67))]
            R1G2B3,

            #[index(Some(68))]
            R1G2B4,

            #[index(Some(69))]
            R1G2B5,

            #[index(Some(70))]
            R1G3B0,

            #[index(Some(71))]
            R1G3B1,

            #[index(Some(72))]
            R1G3B2,

            #[index(Some(73))]
            R1G3B3,

            #[index(Some(74))]
            R1G3B4,

            #[index(Some(75))]
            R1G3B5,

            #[index(Some(76))]
            R1G4B0,

            #[index(Some(77))]
            R1G4B1,

            #[index(Some(78))]
            R1G4B2,

            #[index(Some(79))]
            R1G4B3,

            #[index(Some(80))]
            R1G4B4,

            #[index(Some(81))]
            R1G4B5,

            #[index(Some(82))]
            R1G5B0,

            #[index(Some(83))]
            R1G5B1,

            #[index(Some(84))]
            R1G5B2,

            #[index(Some(85))]
            R1G5B3,

            #[index(Some(86))]
            R1G5B4,

            #[index(Some(87))]
            R1G5B5,

            #[index(Some(88))]
            R2G0B0,

            #[index(Some(89))]
            R2G0B1,

            #[index(Some(90))]
            R2G0B2,

            #[index(Some(91))]
            R2G0B3,

            #[index(Some(92))]
            R2G0B4,

            #[index(Some(93))]
            R2G0B5,

            #[index(Some(94))]
            R2G1B0,

            #[index(Some(95))]
            R2G1B1,

            #[index(Some(96))]
            R2G1B2,

            #[index(Some(97))]
            R2G1B3,

            #[index(Some(98))]
            R2G1B4,

            #[index(Some(99))]
            R2G1B5,

            #[index(Some(100))]
            R2G2B0,

            #[index(Some(101))]
            R2G2B1,

            #[index(Some(102))]
            R2G2B2,

            #[index(Some(103))]
            R2G2B3,

            #[index(Some(104))]
            R2G2B4,

            #[index(Some(105))]
            R2G2B5,

            #[index(Some(106))]
            R2G3B0,

            #[index(Some(107))]
            R2G3B1,

            #[index(Some(108))]
            R2G3B2,

            #[index(Some(109))]
            R2G3B3,

            #[index(Some(110))]
            R2G3B4,

            #[index(Some(111))]
            R2G3B5,

            #[index(Some(112))]
            R2G4B0,

            #[index(Some(113))]
            R2G4B1,

            #[index(Some(114))]
            R2G4B2,

            #[index(Some(115))]
            R2G4B3,

            #[index(Some(116))]
            R2G4B4,

            #[index(Some(117))]
            R2G4B5,

            #[index(Some(118))]
            R2G5B0,

            #[index(Some(119))]
            R2G5B1,

            #[index(Some(120))]
            R2G5B2,

            #[index(Some(121))]
            R2G5B3,

            #[index(Some(122))]
            R2G5B4,

            #[index(Some(123))]
            R2G5B5,

            #[index(Some(124))]
            R3G0B0,

            #[index(Some(125))]
            R3G0B1,

            #[index(Some(126))]
            R3G0B2,

            #[index(Some(127))]
            R3G0B3,

            #[index(Some(128))]
            R3G0B4,

            #[index(Some(129))]
            R3G0B5,

            #[index(Some(130))]
            R3G1B0,

            #[index(Some(131))]
            R3G1B1,

            #[index(Some(132))]
            R3G1B2,

            #[index(Some(133))]
            R3G1B3,

            #[index(Some(134))]
            R3G1B4,

            #[index(Some(135))]
            R3G1B5,

            #[index(Some(136))]
            R3G2B0,

            #[index(Some(137))]
            R3G2B1,

            #[index(Some(138))]
            R3G2B2,

            #[index(Some(139))]
            R3G2B3,

            #[index(Some(140))]
            R3G2B4,

            #[index(Some(141))]
            R3G2B5,

            #[index(Some(142))]
            R3G3B0,

            #[index(Some(143))]
            R3G3B1,

            #[index(Some(144))]
            R3G3B2,

            #[index(Some(145))]
            R3G3B3,

            #[index(Some(146))]
            R3G3B4,

            #[index(Some(147))]
            R3G3B5,

            #[index(Some(148))]
            R3G4B0,

            #[index(Some(149))]
            R3G4B1,

            #[index(Some(150))]
            R3G4B2,

            #[index(Some(151))]
            R3G4B3,

            #[index(Some(152))]
            R3G4B4,

            #[index(Some(153))]
            R3G4B5,

            #[index(Some(154))]
            R3G5B0,

            #[index(Some(155))]
            R3G5B1,

            #[index(Some(156))]
            R3G5B2,

            #[index(Some(157))]
            R3G5B3,

            #[index(Some(158))]
            R3G5B4,

            #[index(Some(159))]
            R3G5B5,

            #[index(Some(160))]
            R4G0B0,

            #[index(Some(161))]
            R4G0B1,

            #[index(Some(162))]
            R4G0B2,

            #[index(Some(163))]
            R4G0B3,

            #[index(Some(164))]
            R4G0B4,

            #[index(Some(165))]
            R4G0B5,

            #[index(Some(166))]
            R4G1B0,

            #[index(Some(167))]
            R4G1B1,

            #[index(Some(168))]
            R4G1B2,

            #[index(Some(169))]
            R4G1B3,

            #[index(Some(170))]
            R4G1B4,

            #[index(Some(171))]
            R4G1B5,

            #[index(Some(172))]
            R4G2B0,

            #[index(Some(173))]
            R4G2B1,

            #[index(Some(174))]
            R4G2B2,

            #[index(Some(175))]
            R4G2B3,

            #[index(Some(176))]
            R4G2B4,

            #[index(Some(177))]
            R4G2B5,

            #[index(Some(178))]
            R4G3B0,

            #[index(Some(179))]
            R4G3B1,

            #[index(Some(180))]
            R4G3B2,

            #[index(Some(181))]
            R4G3B3,

            #[index(Some(182))]
            R4G3B4,

            #[index(Some(183))]
            R4G3B5,

            #[index(Some(184))]
            R4G4B0,

            #[index(Some(185))]
            R4G4B1,

            #[index(Some(186))]
            R4G4B2,

            #[index(Some(187))]
            R4G4B3,

            #[index(Some(188))]
            R4G4B4,

            #[index(Some(189))]
            R4G4B5,

            #[index(Some(190))]
            R4G5B0,

            #[index(Some(191))]
            R4G5B1,

            #[index(Some(192))]
            R4G5B2,

            #[index(Some(193))]
            R4G5B3,

            #[index(Some(194))]
            R4G5B4,

            #[index(Some(195))]
            R4G5B5,

            #[index(Some(196))]
            R5G0B0,

            #[index(Some(197))]
            R5G0B1,

            #[index(Some(198))]
            R5G0B2,

            #[index(Some(199))]
            R5G0B3,

            #[index(Some(200))]
            R5G0B4,

            #[index(Some(201))]
            R5G0B5,

            #[index(Some(202))]
            R5G1B0,

            #[index(Some(203))]
            R5G1B1,

            #[index(Some(204))]
            R5G1B2,

            #[index(Some(205))]
            R5G1B3,

            #[index(Some(206))]
            R5G1B4,

            #[index(Some(207))]
            R5G1B5,

            #[index(Some(208))]
            R5G2B0,

            #[index(Some(209))]
            R5G2B1,

            #[index(Some(210))]
            R5G2B2,

            #[index(Some(211))]
            R5G2B3,

            #[index(Some(212))]
            R5G2B4,

            #[index(Some(213))]
            R5G2B5,

            #[index(Some(214))]
            R5G3B0,

            #[index(Some(215))]
            R5G3B1,

            #[index(Some(216))]
            R5G3B2,

            #[index(Some(217))]
            R5G3B3,

            #[index(Some(218))]
            R5G3B4,

            #[index(Some(219))]
            R5G3B5,

            #[index(Some(220))]
            R5G4B0,

            #[index(Some(221))]
            R5G4B1,

            #[index(Some(222))]
            R5G4B2,

            #[index(Some(223))]
            R5G4B3,

            #[index(Some(224))]
            R5G4B4,

            #[index(Some(225))]
            R5G4B5,

            #[index(Some(226))]
            R5G5B0,

            #[index(Some(227))]
            R5G5B1,

            #[index(Some(228))]
            R5G5B2,

            #[index(Some(229))]
            R5G5B3,

            #[index(Some(230))]
            R5G5B4,

            #[index(Some(231))]
            R5G5B5,

            #[index(Some(232))]
            Grayscale00,

            #[index(Some(233))]
            Grayscale01,

            #[index(Some(234))]
            Grayscale02,

            #[index(Some(235))]
            Grayscale03,

            #[index(Some(236))]
            Grayscale04,

            #[index(Some(237))]
            Grayscale05,

            #[index(Some(238))]
            Grayscale06,

            #[index(Some(239))]
            Grayscale07,

            #[index(Some(240))]
            Grayscale08,

            #[index(Some(241))]
            Grayscale09,

            #[index(Some(242))]
            Grayscale10,

            #[index(Some(243))]
            Grayscale11,

            #[index(Some(244))]
            Grayscale12,

            #[index(Some(245))]
            Grayscale13,

            #[index(Some(246))]
            Grayscale14,

            #[index(Some(247))]
            Grayscale15,

            #[index(Some(248))]
            Grayscale16,

            #[index(Some(249))]
            Grayscale17,

            #[index(Some(250))]
            Grayscale18,

            #[index(Some(251))]
            Grayscale19,

            #[index(Some(252))]
            Grayscale20,

            #[index(Some(253))]
            Grayscale21,

            #[index(Some(254))]
            Grayscale22,

            #[index(Some(255))]
            Grayscale23,

            #[index(None)]
            Reset,
        }

        impl $enum_name {
            /// Iterate through the greyscale members of this enum.
            pub fn iter_grayscale() -> impl Iterator<Item = Self> {
                vec![
                    Self::Grayscale00,
                    Self::Grayscale01,
                    Self::Grayscale02,
                    Self::Grayscale03,
                    Self::Grayscale04,
                    Self::Grayscale05,
                    Self::Grayscale06,
                    Self::Grayscale07,
                    Self::Grayscale08,
                    Self::Grayscale09,
                    Self::Grayscale10,
                    Self::Grayscale11,
                    Self::Grayscale12,
                    Self::Grayscale13,
                    Self::Grayscale14,
                    Self::Grayscale15,
                    Self::Grayscale16,
                    Self::Grayscale17,
                    Self::Grayscale18,
                    Self::Grayscale19,
                    Self::Grayscale20,
                    Self::Grayscale21,
                    Self::Grayscale22,
                    Self::Grayscale23,
                ]
                .into_iter()
            }
        }

        impl IntoANSIEscapeCode for $enum_name {
            fn into_ansi_escape_code(&self) -> ANSIEscapeCode {
                self.index()
                    .map(|colour_idx| {
                        // Colour index
                        ANSIEscapeCode::new(Some($apply_idx), Some(vec![5, colour_idx as i32]), 'm')
                    })
                    .unwrap_or(
                        // Reset
                        ANSIEscapeCode::new(Some($reset_idx), None, 'm'),
                    )
            }
        }

        impl default::Default for $enum_name {
            fn default() -> Self {
                Self::Reset
            }
        }

        impl TryFrom<&ANSIEscapeCode> for $enum_name {
            type Error = ModifierError;

            fn try_from(value: &ANSIEscapeCode) -> Result<Self, Self::Error> {
                // Refactor TryFrom<&Captures<'t>> to use this instead
                if value.end_char != 'm' {
                    return Err(ModifierError::UnexpectedEndCharacter(
                        stringify!($enum_name).to_string(),
                        value.end_char.to_string(),
                    ));
                }

                match (value.code, value.modifiers.len()) {
                    (Some($reset_idx), 0) => Ok(Self::Reset),
                    (Some($apply_idx), 2) => match value.modifiers[0] {
                        5 => u8::try_from(value.modifiers[1])
                            .or(Err(ModifierError::VariantNotFound(
                                stringify!($enum_name).to_string(),
                                value.modifiers.clone(),
                            )))
                            .and_then(|colour_code| {
                                Self::try_from(&Some(colour_code)).or(Err(
                                    ModifierError::VariantNotFound(
                                        stringify!($enum_name).to_string(),
                                        value.modifiers.clone(),
                                    ),
                                ))
                            }),
                        _ => Err(ModifierError::ValueNotRecognised(
                            stringify!($enum_name).to_string(),
                            format!("{:?}", value.modifiers),
                            String::from("Non 5 codes are not allowed."),
                        )),
                    },
                    (Some(idx), 2) if idx != $apply_idx && idx != $reset_idx => {
                        Err(ModifierError::MismatchedANSICode(
                            stringify!($enum_name).to_string(),
                            idx,
                            $apply_idx,
                        ))
                    }
                    (Some(idx), 0) if idx != $apply_idx && idx != $reset_idx => {
                        Err(ModifierError::MismatchedANSICode(
                            stringify!($enum_name).to_string(),
                            idx,
                            $reset_idx,
                        ))
                    }
                    _ => Err(ModifierError::ValueNotRecognised(
                        stringify!($enum_name).to_string(),
                        format!("{:?}:{:?}", value.code, value.modifiers),
                        String::from("Wrong combination of codes."),
                    )),
                }
            }
        }

        builder!($enum_name);
    };
}

color_builder!(Colour, 38, 39);
color_builder!(Background, 48, 49);

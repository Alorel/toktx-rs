use crate::prelude::*;
use delegate_display::DelegateDisplay;
use derive_more::From;
use strum::{EnumString, IntoStaticStr};

use crate::{XY, XYZ};

/// High-quality ASTC format options
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, ConstDefault, Arg)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ASTCOptions {
    ///                Specify block dimension to use for compressing the textures.
    ///                e.g. --astc_blk_d 6x5 for 2D or --astc_blk_d 6x6x6 for 3D.
    ///                6x6 is the default for 2D.
    ///
    ///                    Supported 2D block dimensions are:
    ///
    ///                        4x4: 8.00 bpp         10x5:  2.56 bpp
    ///                        5x4: 6.40 bpp         10x6:  2.13 bpp
    ///                        5x5: 5.12 bpp         8x8:   2.00 bpp
    ///                        6x5: 4.27 bpp         10x8:  1.60 bpp
    ///                        6x6: 3.56 bpp         10x10: 1.28 bpp
    ///                        8x5: 3.20 bpp         12x10: 1.07 bpp
    ///                        8x6: 2.67 bpp         12x12: 0.89 bpp
    ///
    ///                    Supported 3D block dimensions are:
    ///
    ///                        3x3x3: 4.74 bpp       5x5x4: 1.28 bpp
    ///                        4x3x3: 3.56 bpp       5x5x5: 1.02 bpp
    ///                        4x4x3: 2.67 bpp       6x5x5: 0.85 bpp
    ///                        4x4x4: 2.00 bpp       6x6x5: 0.71 bpp
    ///                        5x4x4: 1.60 bpp       6x6x6: 0.59 bpp
    #[arg(rename = "astc_blk_d")]
    #[cfg_attr(feature = "serde", serde(rename = "astc_blk_d", skip_serializing_if = "Option::is_none", default))]
    pub block_dimension: Option<ASTCBlockDimension>,

    /// Specify which encoding mode to use. LDR is the default unless the
    /// input image is 16-bit in which case the default is HDR.
    #[arg(rename = "astc_mode")]
    #[cfg_attr(feature = "serde", serde(rename = "astc_mode", skip_serializing_if = "Option::is_none", default))]
    pub mode: Option<ASTCMode>,

    ///                The quality level configures the quality-performance tradeoff for
    ///                the compressor; more complete searches of the search space
    ///                improve image quality at the expense of compression time. Default
    ///                is 'medium'. The quality level can be set between fastest (0) and
    ///                exhaustive (100) via the following fixed quality presets:
    ///
    ///                    Level      |  Quality
    ///                    ---------- | -----------------------------
    ///                    fastest    | (equivalent to quality =   0)
    ///                    fast       | (equivalent to quality =  10)
    ///                    medium     | (equivalent to quality =  60)
    ///                    thorough   | (equivalent to quality =  98)
    ///                    exhaustive | (equivalent to quality = 100)
    #[arg(rename = "astc_quality")]
    #[cfg_attr(feature = "serde", serde(rename = "astc_quality", skip_serializing_if = "Option::is_none", default))]
    pub quality: Option<u8>,

    /// The codec should optimize for perceptual error, instead of direct
    /// RMS error. This aims to improve perceived image quality, but
    /// typically lowers the measured PSNR score. Perceptual methods are
    /// currently only available for normal maps and RGB color data.
    #[arg(rename = "astc_perceptual")]
    #[cfg_attr(feature = "serde", serde(rename = "astc_perceptual", skip_serializing_if = "crate::is_false", default))]
    pub perceptual: bool,
}

impl ASTCOptions {
    /// `--astc_quality` preset
    pub const QUALITY_FASTEST: u8 = 0;

    /// `--astc_quality` preset
    pub const QUALITY_FAST: u8 = 10;

    /// `--astc_quality` preset
    pub const QUALITY_MEDIUM: u8 = 60;

    /// `--astc_quality` preset
    pub const QUALITY_THOROUGH: u8 = 98;

    /// `--astc_quality` preset
    pub const QUALITY_EXHAUSTIVE: u8 = 100;
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, DelegateDisplay, Arg, From)]
#[arg(to_string)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum ASTCBlockDimension {
    XYZ(XYZ<u8>),
    XY(XY<u8>),
}

/// Astc encoding mode
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, IntoStaticStr, EnumString)]
#[strum(serialize_all = "lowercase")]
#[allow(missing_docs)]
pub enum ASTCMode {
    Ldr,
    Hdr,
}

arg_static_str!(ASTCMode);

cfg_if! {
    if #[cfg(feature = "serde")] {
        serde_util!(serialise ASTCMode as static_str);
        serde_util!(deserialise ASTCMode as parse with "ASTCMode");
    }
}

use crate::prelude::*;

/// High-quality transcodable UASTC format.
#[derive(Debug, Clone, PartialEq, Default, ConstDefault, Arg)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UASTCOptions {
    /// This optional parameter selects a speed vs quality
    /// tradeoff as shown in the following table:
    ///
    /// Level |  Speed    | Quality
    /// ----- | --------- | -------
    /// 0   |  Fastest  | 43.45dB
    /// 1   |  Faster   | 46.49dB
    /// 2   |  Default  | 47.47dB
    /// 3   |  Slower   | 48.01dB
    /// 4   | Very slow | 48.24dB
    ///
    /// You are strongly encouraged to also specify `--zcmp` to losslessly
    /// compress the UASTC data. This and any LZ-style compression can
    /// be made more effective by conditioning the UASTC texture data
    /// using the Rate Distortion Optimization (RDO) post-process stage.
    #[arg(rename = "uastc_quality")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_quality", skip_serializing_if = "Option::is_none", default))]
    pub quality: Option<UASTCQuality>,

    /// Enable UASTC RDO post-processing and optionally set UASTC RDO
    /// quality scalar (lambda) to @e lambda.  Lower values yield higher
    /// quality/larger LZ compressed files, higher values yield lower
    /// quality/smaller LZ compressed files. A good range to try is
    /// `0.25..10`. For normal maps a good range is `0.25..0.75`. The full
    /// range is `0.001..=10.0`. Default is 1.0.
    ///
    /// Note that previous versions used the `--uastc_rdo_q` option which
    /// was removed because the RDO algorithm changed.
    #[arg(rename = "uastc_rdo_l")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_rdo_l", skip_serializing_if = "Option::is_none", default))]
    pub rdo_lambda: Option<f32>,

    /// Set UASTC RDO dictionary size in bytes. Default is 4096. Lower
    /// values=faster, but give less compression. Range is `64..=65536`.
    #[arg(rename = "uastc_rdo_d")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_rdo_d", skip_serializing_if = "Option::is_none", default))]
    pub rdo_dictionary_size: Option<u16>,

    /// Set UASTC RDO max smooth block error scale. Range is `1.0..=300.0`.
    /// Default is 10.0, 1.0 is disabled. Larger values suppress more
    /// artifacts (and allocate more bits) on smooth blocks.
    #[arg(rename = "uastc_rdo_b")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_rdo_b", skip_serializing_if = "Option::is_none", default))]
    pub rdo_block_error_scale: Option<f32>,

    /// Set UASTC RDO max smooth block standard deviation. Range is
    /// `.01..=65536.0`. Default is 18.0. Larger values expand the range
    /// of blocks considered smooth.
    #[arg(rename = "uastc_rdo_s")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_rdo_s", skip_serializing_if = "Option::is_none", default))]
    pub rdo_std_dev: Option<f32>,

    /// Do not favor simpler UASTC modes in RDO mode
    #[arg(rename = "uastc_rdo_f")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_rdo_f", skip_serializing_if = "crate::is_false", default))]
    pub rdo_f: bool,

    /// Disable RDO multithreading (slightly higher compression, deterministic).
    #[arg(rename = "uastc_rdo_m")]
    #[cfg_attr(feature = "serde", serde(rename = "uastc_rdo_m", skip_serializing_if = "crate::is_false", default))]
    pub rdo_no_multithreading: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Arg)]
#[repr(u8)]
#[arg(as_repr)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(into = "u8", try_from = "u8"))]
pub enum UASTCQuality {
    Fastest = 0,
    Faster = 1,
    Default = 2,
    Slower = 3,
    VerySlow = 4,
}

impl TryFrom<u8> for UASTCQuality {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > (UASTCQuality::VerySlow as u8) {
            Err("Only values 0..=4 are valid")
        } else {
            Ok(unsafe { std::mem::transmute(value) })
        }
    }
}

impl From<UASTCQuality> for u8 {
    #[inline]
    fn from(value: UASTCQuality) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod test {
    use super::UASTCQuality;

    #[test]
    fn to_and_from_repr() {
        let cases = [
            (UASTCQuality::Fastest, 0u8),
            (UASTCQuality::Faster, 1),
            (UASTCQuality::Default, 2),
            (UASTCQuality::Slower, 3),
            (UASTCQuality::VerySlow, 4),
        ];

        for (inst, repr) in cases {
            assert_eq!((inst as u8), repr, "inst as u8 ({})", repr);
            assert_eq!(UASTCQuality::try_from(repr), Ok(inst), "try_from ({})", repr);
        }
    }
}

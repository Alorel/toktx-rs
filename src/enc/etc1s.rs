use crate::prelude::*;

/// ETC1S / BasisLZ supercompression options
#[derive(Debug, Clone, PartialEq, Default, ConstDefault, Arg)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ETC1SOptions {
    /// ETC1S / BasisLZ compression level, an encoding speed vs. quality
    /// tradeoff. Range is [0,5], default is 1. Higher values are slower
    /// but give higher quality.
    #[arg(rename = "clevel")]
    #[cfg_attr(feature = "serde", serde(rename = "clevel", skip_serializing_if = "Option::is_none", default))]
    pub compression_level: Option<f32>,

    /// ETC1S / BasisLZ quality level. Range is [1,255]. Lower gives
    /// better compression/lower quality/faster. Higher gives less
    /// compression/higher quality/slower. --qlevel automatically
    /// determines values for --max_endpoints, --max-selectors,
    /// --endpoint_rdo_threshold and --selector_rdo_threshold for the
    /// target quality level. Setting these options overrides the values
    /// determined by -qlevel which defaults to 128 if neither it nor
    /// both of --max_endpoints and --max_selectors have been set.
    ///
    /// Note that both of --max_endpoints and --max_selectors
    /// must be set for them to have any effect. If all three options
    /// are set, a warning will be issued that --qlevel will be ignored.
    ///
    /// Note also that --qlevel will only determine values for
    /// --endpoint_rdo_threshold and --selector_rdo_threshold when
    /// its value exceeds 128, otherwise their defaults will be used.
    #[arg(rename = "qlevel")]
    #[cfg_attr(feature = "serde", serde(rename = "qlevel", skip_serializing_if = "Option::is_none", default))]
    pub quality_level: Option<u8>,

    /// Manually set the maximum number of color endpoint clusters. Range
    /// is [1,16128]. Default is 0, unset.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub max_endpoints: Option<u16>,

    /// Set endpoint RDO quality threshold. The default is 1.25. Lower
    /// is higher quality but less quality per output bit (try
    /// `1.0.=3.0`). This will override the value chosen by `qlevel`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub endpoint_rdo_threshold: Option<f32>,

    /// Manually set the maximum number of color selector clusters from
    /// [1,16128]. Default is 0, unset.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub max_selectors: Option<u16>,

    /// Set selector RDO quality threshold. The default is 1.25. Lower
    /// is higher quality but less quality per output bit (try
    /// `1.0.=3.0`). This will override the value chosen by `qlevel`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub selector_rdo_threshold: Option<f32>,

    /// Disable endpoint rate distortion optimizations. Slightly faster,
    /// less noisy output, but lower quality per output bit. Default is
    /// to do endpoint RDO.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub no_endpoint_rdo: bool,

    /// Disable selector rate distortion optimizations. Slightly faster,
    /// less noisy output, but lower quality per output bit. Default is
    /// to do selector RDO.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub no_selector_rdo: bool,
}

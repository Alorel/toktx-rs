use std::ffi::OsString;

use crate::*;

/// KTX texture converter. See [crate-level docs](crate).
#[derive(Debug, Clone, PartialEq, Default, ConstDefault, Arg)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToKtx {
    /// `--2d`
    ///
    /// If the image height is 1, by default a KTX file for a 1D
    /// texture is created. With this option one for a 2D texture is
    /// created instead.
    #[arg(rename = "2d")]
    #[cfg_attr(feature = "serde", serde(rename = "2d", skip_serializing_if = "crate::is_false", default))]
    pub two_dee: bool,

    /// `--automipmap`
    ///
    /// Causes the KTX file to be marked to request generation of a mipmap pyramid when the file is
    /// loaded. This option is mutually exclusive with `genmipmap`, `levels` and `mipmap`.
    #[arg(rename = "automipmap")]
    #[cfg_attr(feature = "serde", serde(rename = "automipmap", skip_serializing_if = "crate::is_false", default))]
    pub auto_mipmap: bool,

    /// `--cubemap`
    ///
    /// KTX file is for a cubemap. At least 6 <infile>s must be provided,
    /// more if --mipmap is also specified. Provide the images in the
    /// order +X, -X, +Y, -Y, +Z, -Z where the arrangement is a
    /// left-handed coordinate system with +Y up. So if you're facing +Z,
    /// -X will be on your left and +X on your right. If `layers` > 1
    /// is specified, provide the faces for layer 0 first then for
    /// layer 1, etc. Images must have an upper left origin so
    /// `lower_left_maps_to_s0t0` is ignored with this option.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub cubemap: bool,

    /// KTX file is for a 3D texture with a depth of number where
    /// number > 0. Provide the file(s) for z=0 first then those for
    /// z=1, etc. It is an error to specify this together with
    /// `layers` or `cubemap`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub depth: Option<u32>,

    /// Causes mipmaps to be generated for each input file. This option
    /// is mutually exclusive with `automipmap` and `mipmap`. When set
    /// the following mipmap-generation related options become valid,
    /// otherwise they are ignored.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub genmipmap: bool,

    /// Filter to use when generating mipmaps
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub filter: Option<Filter>,

    /// The filter scale to use.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub fscale: Option<f32>,

    /// How to sample pixels near the image boundaries
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub wmode: Option<WMode>,

    /// KTX file is for an array texture with number of layers
    /// where number > 0. Provide the file(s) for layer 0 first then
    /// those for layer 1, etc. It is an error to specify this
    /// together with `depth`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub layers: Option<u32>,

    /// KTX file is for a mipmap pyramid with <number> of levels rather
    /// than a full pyramid. number must be > 1 and <= the maximum number
    /// of levels determined from the size of the base image. This option
    /// is mutually exclusive with @b `automipmap`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub levels: Option<u32>,

    /// KTX file is for a mipmap pyramid with one infile being explicitly
    /// provided for each level. Provide the images in the order of layer
    /// then face or depth slice then level with the base-level image
    /// first then in order down to the 1x1 image or the level specified
    /// by --levels.  This option is mutually exclusive with --automipmap
    /// and --genmipmap. Note that this ordering differs from that in the
    /// created texture as it is felt to be more user-friendly.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub mipmap: bool,

    /// Do not write KTXorientation metadata into the output file.
    /// Use of this option is not recommended.
    #[arg(rename = "nometadata")]
    #[cfg_attr(feature = "serde", serde(rename = "nometadata", skip_serializing_if = "crate::is_false", default))]
    pub no_metadata: bool,

    /// Silence warnings which are issued when certain transformations
    /// are performed on input images.
    #[arg(rename = "nowarn")]
    #[cfg_attr(feature = "serde", serde(rename = "nowarn", skip_serializing_if = "crate::is_false", default))]
    pub no_warn: bool,

    /// Map the logical upper left corner of the image to s0,t0.
    /// Although opposite to the OpenGL convention, this is the DEFAULT
    /// BEHAVIOUR. netpbm and PNG files have an upper left origin so
    /// this option does not flip the input images. When this option is
    /// in effect, toktx writes a KTXorientation value of S=r,T=d into
    /// the output file to inform loaders of the logical orientation. If
    /// an OpenGL {,ES} loader ignores the orientation value, the image
    /// will appear upside down.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub upper_left_maps_to_s0t0: bool,

    /// Map the logical lower left corner of the image to s0,t0.
    /// This causes the input netpbm and PNG images to be flipped
    /// vertically to a lower-left origin. When this option is in effect,
    /// toktx writes a KTXorientation value of S=r,T=u into the output
    /// file to inform loaders of the logical orientation. If a Vulkan
    /// loader ignores the orientation value, the image will appear
    /// upside down. This option is ignored with --cubemap.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub lower_left_maps_to_s0t0: bool,

    /// Force the created texture to have the specified transfer
    /// function. If this is specified, implicit or explicit color space
    /// information from the input file(s) will be ignored and no color
    /// transformation will be performed. USE WITH CAUTION preferably
    /// only when you know the file format information is wrong.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub assign_oetf: Option<TransferFunction>,

    /// Force the created texture to have the specified primaries. If
    /// this is specified, implicit or explicit color space information
    /// from the input file(s) will be ignored and no color
    /// transformation will be performed. USE WITH CAUTION preferably
    /// only when you know the file format information is wrong.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub assign_primaries: Option<Primaries>,

    /// Convert the input images to the specified transfer function, if
    /// the current transfer function is different. If both this and
    /// --assign_oetf are specified, conversion will be performed from
    /// the assigned transfer function to the transfer function specified
    /// by this option, if different.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub convert_oetf: Option<TransferFunction>,

    /// Add swizzle metadata to the file being created. swizzle has the
    /// same syntax as the parameter for --input_swizzle. Not recommended
    /// for use with block-cmpressed textures, including Basis Universal
    /// formats, because something like `rabb` may yield drastically
    /// different error metrics if done after compression.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub swizzle: Option<Swizzle>,

    /// Specify the number of components in the created texture. type is
    /// one of the following strings: @c R, @c RG, @c RGB or @c RGBA.
    /// Excess input components will be dropped. Output components with
    /// no mapping from the input will be set to 0 or, if the alpha
    /// component, 1.0.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub target_type: Option<TargetType>,

    /// Resize images to @e width X @e height. This should not be used
    /// with @b`--mipmap` as it would resize all the images to the same
    /// size. Resampler options can be set via `filter` and `fscale`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub resize: Option<XY<u32>>,

    /// Scale images by <value> as they are read. Resampler options can
    /// be set via `filter` and `fscale`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub scale: Option<f32>,

    /// Output in KTX2 format. Default is KTX.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_default", default))]
    pub output_format: OutputFormat,

    /// Encoding options
    #[arg(rename = "encode")]
    #[cfg_attr(feature = "serde", serde(rename = "encode", skip_serializing_if = "Option::is_none", default))]
    pub encoding: Option<enc::Encoding>,

    /// Swizzle the input components according to swizzle
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub input_swizzle: Option<Swizzle>,

    /// Only valid for linear textures with two or more components. If
    /// the input texture has three or four linear components it is
    /// assumed to be a three component linear normal map storing unit
    /// length normals as `(R=X, G=Y, B=Z)`. A fourth component will be
    /// ignored. The map will be converted to a two component X+Y normal
    /// map stored as `(RGB=X, A=Y)` prior to encoding. If unsure that
    /// your normals are unit length, use @b `--normalize`. If the input
    /// has 2 linear components it is assumed to be an X+Y map of unit
    /// normals.
    ///
    /// The Z component can be recovered programmatically in shader
    /// code by using the equations:
    ///
    /// `nml.xy = texture(...).ga;              // Load in [0,1]`
    ///
    /// `nml.xy = nml.xy * 2.0 - 1.0;           // Unpack to [-1,1]`
    ///
    /// `nml.z = sqrt(1 - dot(nml.xy, nml.xy)); // Compute Z`
    ///
    /// Encoding is optimized for normal maps. For ASTC encoding,
    /// `--encode astc`, encoder parameters are tuned for better quality
    /// on normal maps. .  For ETC1S encoding, '--encode etc1s',i RDO is
    /// disabled (no selector RDO, no endpoint RDO) to provide better
    /// quality.
    ///
    /// You can prevent conversion of the normal map to two components
    /// by specifying `--input_swizzle rgb1`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub normal_mode: bool,

    /// Normalize input normals to have a unit length. Only valid for
    /// linear textures with 2 or more components. For 2-component inputs
    /// 2D unit normals are calculated. Do not use these 2D unit normals
    /// to generate X+Y normals for `--normal_mode`. For 4-component inputs
    /// a 3D unit normal is calculated. 1.0 is used for the value of the
    /// 4th component.
    #[arg(rename = "normalize")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub normalise: bool,

    /// Forbid use of the SSE instruction set. Ignored if CPU does not
    /// support SSE. Only the Basis Universal compressor uses SSE.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "crate::is_false", default))]
    pub no_sse: bool,

    /// Supercompress the data with Zstandard. Implies [`--t2`](OutputFormat::KTX2). Can be used
    /// with data in any format except [`ETC1S / BasisLZ`](enc::Encoding::ETC1S). Most
    /// effective with RDO-conditioned UASTC or uncompressed formats. The
    /// optional compressionLevel range is 1 - 22 and the default is 3.
    /// Lower values=faster but give less compression. Values above 20
    /// should be used with caution as they require more memory.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub zcmp: Option<u8>,

    /// Explicitly set the number of threads to use during compression.
    /// By default, ETC1S / BasisLZ and ASTC compression will use the
    /// number of threads reported by thread::hardware_concurrency or 1
    /// if value returned is 0.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub threads: Option<u16>,

    /// Path to the `toktx` CLI tool
    #[arg(skip)]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
    pub path_to_toktx: Option<OsString>,
}

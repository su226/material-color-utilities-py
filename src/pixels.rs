use material_colors::color::Rgb;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::color::{PyRgb, rgb_from_py};

/// Byte order of an 8-bit RGB/RGBA pixel buffer.
///
/// The variant names describe the order of the bytes in memory, not the
/// logical 32-bit word. For example, `ARGB` means `[A, R, G, B]`, while
/// Cairo's little-endian `ARGB32` surface is actually laid out as
/// `[B, G, R, A]` and should use `BGRA`.
#[pyclass(name = "PixelFormat", from_py_object)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyPixelFormat {
    Rgb,
    Bgr,
    Rgba,
    Bgra,
    Argb,
    Abgr,
}

impl PyPixelFormat {
    #[inline]
    const fn channels(self) -> usize {
        match self {
            Self::Rgb | Self::Bgr => 3,
            Self::Rgba | Self::Bgra | Self::Argb | Self::Abgr => 4,
        }
    }

    /// Byte indices of red, green and blue.
    #[inline]
    const fn rgb_indices(self) -> [usize; 3] {
        match self {
            Self::Rgb | Self::Rgba => [0, 1, 2],
            Self::Bgr | Self::Bgra => [2, 1, 0],
            Self::Argb => [1, 2, 3],
            Self::Abgr => [3, 2, 1],
        }
    }

    /// Byte index of alpha, if this format has one.
    #[inline]
    const fn alpha_index(self) -> Option<usize> {
        match self {
            Self::Rgb | Self::Bgr => None,
            Self::Rgba | Self::Bgra => Some(3),
            Self::Argb | Self::Abgr => Some(0),
        }
    }
}

/// Converts Python `(r, g, b)` tuples into the crate's pixels type.
#[inline]
pub(crate) fn pixels_from_list(pixels: Vec<PyRgb>) -> Vec<Rgb> {
    pixels.into_iter().map(rgb_from_py).collect()
}

/// Iterates pixels of a packed or strided buffer.
///
/// The stride branch is evaluated once, outside the pixel loop. The macro is
/// used instead of a closure so the no-alpha and alpha-filtering loops can
/// avoid an `Option`/closure check per pixel.
macro_rules! for_each_pixel {
    ($data:expr, $channels:expr, $width:expr, $stride:expr, |$pixel:ident| $body:block) => {
        if let (Some(stride), Some(width)) = ($stride, $width) {
            let row_bytes = width * $channels;

            for row in $data.chunks_exact(stride) {
                for $pixel in row[..row_bytes].chunks_exact($channels) {
                    $body
                }
            }
        } else {
            for $pixel in $data.chunks_exact($channels) {
                $body
            }
        }
    };
}

/// Converts a raw 8-bit pixel buffer into the crate's pixels type.
///
/// All layout checks and the alpha/premultiplication strategy are selected
/// once, outside the pixel loop. The loop itself only performs the checks
/// that are actually required by the chosen mode.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pixels_from_bytes(
    data: &[u8],
    format: PyPixelFormat,
    alpha_threshold: u8,
    premultiplied: bool,
    width: Option<usize>,
    stride: Option<usize>,
) -> PyResult<Vec<Rgb>> {
    let channels = format.channels();
    let [ri, gi, bi] = format.rgb_indices();
    let alpha = format.alpha_index();

    // `alpha_threshold` is meaningless without an alpha channel. Ignore it
    // so the Python-level default (128, which is useful for RGBA images) can
    // also be used for plain RGB buffers.
    if alpha.is_none() && premultiplied {
        return Err(PyValueError::new_err(
            "premultiplied alpha requires a 4-channel pixel format",
        ));
    }

    if stride.is_some() && width.is_none() {
        return Err(PyValueError::new_err(
            "width is required when stride is specified",
        ));
    }
    if width == Some(0) {
        return Err(PyValueError::new_err("width must be greater than zero"));
    }
    if stride == Some(0) {
        return Err(PyValueError::new_err("stride must be greater than zero"));
    }

    let row_bytes = match width {
        Some(width) => Some(
            width
                .checked_mul(channels)
                .ok_or_else(|| PyValueError::new_err("width is too large"))?,
        ),
        None => None,
    };

    if let (Some(stride), Some(row_bytes)) = (stride, row_bytes) {
        if stride < row_bytes {
            return Err(PyValueError::new_err(format!(
                "stride ({stride}) is smaller than one row ({row_bytes})"
            )));
        }
        if !data.len().is_multiple_of(stride) {
            return Err(PyValueError::new_err(
                "pixel buffer length is not a multiple of stride",
            ));
        }
    } else if let Some(row_bytes) = row_bytes {
        if !data.len().is_multiple_of(row_bytes) {
            return Err(PyValueError::new_err(
                "pixel buffer length is not a multiple of one row",
            ));
        }
    } else if !data.len().is_multiple_of(channels) {
        return Err(PyValueError::new_err(format!(
            "pixel buffer length is not a multiple of {channels}"
        )));
    }

    let capacity = if let (Some(stride), Some(width)) = (stride, width) {
        (data.len() / stride) * width
    } else {
        data.len() / channels
    };

    let mut result = Vec::with_capacity(capacity);

    match (alpha, premultiplied, alpha_threshold) {
        // No alpha channel, or a four-channel format whose alpha byte is
        // ignored (RGBX/BGRX/XRGB/XBGR).
        (None, _, _) | (Some(_), false, 0) => {
            for_each_pixel!(data, channels, width, stride, |pixel| {
                result.push(Rgb::new(pixel[ri], pixel[gi], pixel[bi]));
            });
        }
        // Straight alpha with a threshold.
        (Some(ai), false, threshold) => {
            for_each_pixel!(data, channels, width, stride, |pixel| {
                if pixel[ai] >= threshold {
                    result.push(Rgb::new(pixel[ri], pixel[gi], pixel[bi]));
                }
            });
        }
        // Premultiplied alpha. Even with a zero threshold, alpha == 0 must be
        // skipped because its color cannot be recovered.
        (Some(ai), true, threshold) => {
            for_each_pixel!(data, channels, width, stride, |pixel| {
                let alpha = pixel[ai];

                if alpha != 0 && alpha >= threshold {
                    result.push(Rgb::new(
                        unpremultiply(pixel[ri], alpha),
                        unpremultiply(pixel[gi], alpha),
                        unpremultiply(pixel[bi], alpha),
                    ));
                }
            });
        }
    }

    Ok(result)
}

#[inline]
fn unpremultiply(value: u8, alpha: u8) -> u8 {
    let alpha = u16::from(alpha);

    ((u16::from(value) * 255 + alpha / 2) / alpha).min(255) as u8
}

from collections.abc import Buffer, Sequence
from typing import Never, overload

try:
    from PIL.Image import Image
except ImportError:
    Image = Never

from material_color_utilities._core import SRGB_TO_XYZ as SRGB_TO_XYZ
from material_color_utilities._core import WHITE_POINT_D65 as WHITE_POINT_D65
from material_color_utilities._core import XYZ_TO_SRGB as XYZ_TO_SRGB
from material_color_utilities._core import ArgbColor as ArgbColor
from material_color_utilities._core import Cam16 as Cam16
from material_color_utilities._core import ColorSpec as ColorSpec
from material_color_utilities._core import ColorSpec2021 as ColorSpec2021
from material_color_utilities._core import ColorSpec2025 as ColorSpec2025
from material_color_utilities._core import ColorSpec2026 as ColorSpec2026
from material_color_utilities._core import ContrastCurve as ContrastCurve
from material_color_utilities._core import CorePalette as CorePalette
from material_color_utilities._core import CorePalettes as CorePalettes
from material_color_utilities._core import DeltaConstraint as DeltaConstraint
from material_color_utilities._core import DynamicColor as DynamicColor
from material_color_utilities._core import DynamicScheme as DynamicScheme
from material_color_utilities._core import Hct as Hct
from material_color_utilities._core import (
    MaterialDynamicColors as MaterialDynamicColors,
)
from material_color_utilities._core import Palette as Palette
from material_color_utilities._core import Palettes as Palettes
from material_color_utilities._core import PixelFormat as PixelFormat
from material_color_utilities._core import Platform as Platform
from material_color_utilities._core import PointProviderLab as PointProviderLab
from material_color_utilities._core import QuantizerResult as QuantizerResult
from material_color_utilities._core import Random as Random
from material_color_utilities._core import Scheme as Scheme
from material_color_utilities._core import SchemeCmf as SchemeCmf
from material_color_utilities._core import SchemeContent as SchemeContent
from material_color_utilities._core import SchemeExpressive as SchemeExpressive
from material_color_utilities._core import SchemeFidelity as SchemeFidelity
from material_color_utilities._core import SchemeFruitSalad as SchemeFruitSalad
from material_color_utilities._core import SchemeMonochrome as SchemeMonochrome
from material_color_utilities._core import SchemeNeutral as SchemeNeutral
from material_color_utilities._core import SchemeRainbow as SchemeRainbow
from material_color_utilities._core import SchemeResolver as SchemeResolver
from material_color_utilities._core import SchemeTonalSpot as SchemeTonalSpot
from material_color_utilities._core import SchemeVibrant as SchemeVibrant
from material_color_utilities._core import Schemes as Schemes
from material_color_utilities._core import SpecVersion as SpecVersion
from material_color_utilities._core import Theme as Theme
from material_color_utilities._core import ThemeBuilder as ThemeBuilder
from material_color_utilities._core import TonalPalette as TonalPalette
from material_color_utilities._core import ToneDeltaPair as ToneDeltaPair
from material_color_utilities._core import TonePolarity as TonePolarity
from material_color_utilities._core import Variant as Variant
from material_color_utilities._core import ViewingConditions as ViewingConditions
from material_color_utilities._core import _extract_color_raw, _extract_color_rgb
from material_color_utilities._core import _quantize_celebi_raw, _quantize_celebi_rgb
from material_color_utilities._core import _quantize_map_raw, _quantize_map_rgb
from material_color_utilities._core import _quantize_wsmeans_raw, _quantize_wsmeans_rgb
from material_color_utilities._core import _quantize_wu_raw, _quantize_wu_rgb
from material_color_utilities._core import cam16_ucs as cam16_ucs
from material_color_utilities._core import darker as darker
from material_color_utilities._core import darker_unsafe as darker_unsafe
from material_color_utilities._core import delinearized as delinearized
from material_color_utilities._core import difference_degrees as difference_degrees
from material_color_utilities._core import fix_if_disliked as fix_if_disliked
from material_color_utilities._core import harmonize as harmonize
from material_color_utilities._core import hct_hue as hct_hue
from material_color_utilities._core import hex_to_rgb as hex_to_rgb
from material_color_utilities._core import int_to_rgb as int_to_rgb
from material_color_utilities._core import is_disliked as is_disliked
from material_color_utilities._core import lab_to_rgb as lab_to_rgb
from material_color_utilities._core import lerp as lerp
from material_color_utilities._core import lighter as lighter
from material_color_utilities._core import lighter_unsafe as lighter_unsafe
from material_color_utilities._core import linear_rgb_to_rgb as linear_rgb_to_rgb
from material_color_utilities._core import linearized as linearized
from material_color_utilities._core import lstar_from_y as lstar_from_y
from material_color_utilities._core import lstar_to_rgb as lstar_to_rgb
from material_color_utilities._core import matrix_multiply as matrix_multiply
from material_color_utilities._core import ratio_of_tones as ratio_of_tones
from material_color_utilities._core import rgb_to_argb_int as rgb_to_argb_int
from material_color_utilities._core import rgb_to_hex as rgb_to_hex
from material_color_utilities._core import rgb_to_int as rgb_to_int
from material_color_utilities._core import rgb_to_lab as rgb_to_lab
from material_color_utilities._core import rgb_to_lstar as rgb_to_lstar
from material_color_utilities._core import rgb_to_xyz as rgb_to_xyz
from material_color_utilities._core import rotate_direction as rotate_direction
from material_color_utilities._core import (
    sanitize_degrees_double as sanitize_degrees_double,
)
from material_color_utilities._core import sanitize_degrees_int as sanitize_degrees_int
from material_color_utilities._core import signum as signum
from material_color_utilities._core import xyz_to_rgb as xyz_to_rgb
from material_color_utilities._core import y_from_lstar as y_from_lstar

RGB = tuple[int, int, int]
"""3-tuple of uint8 contains sRGB red, green and blue data."""

LinearRGB = tuple[float, float, float]
"""3-tuple of float64 contains linear red, green and blue data."""

XYZ = tuple[float, float, float]
"""3-tuple of float64 contains CIE XYZ data."""

Lab = tuple[float, float, float]
"""3-tuple of float64 contains CIE Lab data."""


@overload
def extract_color(
    image: Buffer,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> RGB: ...


@overload
def extract_color(image: Sequence[RGB]) -> RGB: ...


@overload
def extract_color(image: Image, *, alpha_threshold: int = 128) -> RGB: ...


def extract_color(
    image: Image | Buffer | Sequence[RGB],
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> RGB:
    """
    Extract primary color from an image or a pixel buffer.

    :param image: Pillow image in RGB, RGBX or RGBA mode; a buffer of raw
                  8-bit pixels; or a sequence of ``(r, g, b)`` tuples.
    :param pixel_format: Byte order of ``image`` when it is a buffer.
    :param alpha_threshold: Cut off alpha below this value for 4-channel
                            formats. Ignored when the format has no alpha.
    :param premultiplied: Whether a 4-channel buffer is premultiplied.
    :param width: Width of a strided buffer, in pixels.
    :param stride: Bytes per row, including padding.
    :return: 3-tuple of (R, G, B)
    """
    if isinstance(image, Buffer):
        return _extract_color_raw(
            image,
            pixel_format,
            alpha_threshold,
            premultiplied,
            width,
            stride,
        )

    if isinstance(image, Sequence):
        return _extract_color_rgb(image)

    if image.mode in ("RGB", "RGBX"):
        return _extract_color_raw(image.tobytes("raw", "rgb"))
    elif image.mode == "RGBA":
        return _extract_color_raw(
            image.tobytes(),
            PixelFormat.Rgba,
            alpha_threshold,
        )
    else:
        raise NotImplementedError(f"Unsupported mode: {image.mode}")


@overload
def quantize_celebi(
    pixels: Buffer,
    max_colors: int,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult: ...


@overload
def quantize_celebi(pixels: Sequence[RGB], max_colors: int) -> QuantizerResult: ...


@overload
def quantize_celebi(
    pixels: Image,
    max_colors: int,
    *,
    alpha_threshold: int = 128,
) -> QuantizerResult: ...


def quantize_celebi(
    pixels: Image | Buffer | Sequence[RGB],
    max_colors: int,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult:
    """Quantize pixels with the Celebi (Wu + WSMeans) quantizer."""
    if isinstance(pixels, Buffer):
        return _quantize_celebi_raw(
            pixels,
            max_colors,
            pixel_format,
            alpha_threshold,
            premultiplied,
            width,
            stride,
        )

    if isinstance(pixels, Sequence):
        return _quantize_celebi_rgb(pixels, max_colors)

    if pixels.mode in ("RGB", "RGBX"):
        return _quantize_celebi_raw(pixels.tobytes("raw", "rgb"), max_colors)
    elif pixels.mode == "RGBA":
        return _quantize_celebi_raw(
            pixels.tobytes(),
            max_colors,
            PixelFormat.Rgba,
            alpha_threshold,
        )
    else:
        raise NotImplementedError(f"Unsupported mode: {pixels.mode}")


@overload
def quantize_map(
    pixels: Buffer,
    max_colors: int,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult: ...


@overload
def quantize_map(pixels: Sequence[RGB], max_colors: int) -> QuantizerResult: ...


@overload
def quantize_map(
    pixels: Image,
    max_colors: int,
    *,
    alpha_threshold: int = 128,
) -> QuantizerResult: ...


def quantize_map(
    pixels: Image | Buffer | Sequence[RGB],
    max_colors: int,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult:
    """Count the occurrences of each distinct pixel color."""
    if isinstance(pixels, Buffer):
        return _quantize_map_raw(
            pixels,
            max_colors,
            pixel_format,
            alpha_threshold,
            premultiplied,
            width,
            stride,
        )

    if isinstance(pixels, Sequence):
        return _quantize_map_rgb(pixels, max_colors)

    if pixels.mode in ("RGB", "RGBX"):
        return _quantize_map_raw(pixels.tobytes("raw", "rgb"), max_colors)
    elif pixels.mode == "RGBA":
        return _quantize_map_raw(
            pixels.tobytes(),
            max_colors,
            PixelFormat.Rgba,
            alpha_threshold,
        )
    else:
        raise NotImplementedError(f"Unsupported mode: {pixels.mode}")


@overload
def quantize_wsmeans(
    input_pixels: Buffer,
    max_colors: int,
    starting_clusters: Sequence[RGB],
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult: ...


@overload
def quantize_wsmeans(
    input_pixels: Sequence[RGB],
    max_colors: int,
    starting_clusters: Sequence[RGB],
) -> QuantizerResult: ...


@overload
def quantize_wsmeans(
    input_pixels: Image,
    max_colors: int,
    starting_clusters: Sequence[RGB],
    *,
    alpha_threshold: int = 128,
) -> QuantizerResult: ...


def quantize_wsmeans(
    input_pixels: Image | Buffer | Sequence[RGB],
    max_colors: int,
    starting_clusters: Sequence[RGB],
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult:
    """Quantize pixels with the weighted squared-mean (k-means) quantizer."""
    if isinstance(input_pixels, Buffer):
        return _quantize_wsmeans_raw(
            input_pixels,
            max_colors,
            starting_clusters,
            pixel_format,
            alpha_threshold,
            premultiplied,
            width,
            stride,
        )

    if isinstance(input_pixels, Sequence):
        return _quantize_wsmeans_rgb(input_pixels, max_colors, starting_clusters)

    if input_pixels.mode in ("RGB", "RGBX"):
        return _quantize_wsmeans_raw(
            input_pixels.tobytes("raw", "rgb"), max_colors, starting_clusters
        )
    elif input_pixels.mode == "RGBA":
        return _quantize_wsmeans_raw(
            input_pixels.tobytes(),
            max_colors,
            starting_clusters,
            PixelFormat.Rgba,
            alpha_threshold,
        )
    else:
        raise NotImplementedError(f"Unsupported mode: {input_pixels.mode}")


@overload
def quantize_wu(
    pixels: Buffer,
    max_colors: int,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult: ...


@overload
def quantize_wu(pixels: Sequence[RGB], max_colors: int) -> QuantizerResult: ...


@overload
def quantize_wu(
    pixels: Image,
    max_colors: int,
    *,
    alpha_threshold: int = 128,
) -> QuantizerResult: ...


def quantize_wu(
    pixels: Image | Buffer | Sequence[RGB],
    max_colors: int,
    *,
    pixel_format: PixelFormat = PixelFormat.Rgb,
    alpha_threshold: int = 128,
    premultiplied: bool = False,
    width: int | None = None,
    stride: int | None = None,
) -> QuantizerResult:
    """Quantize pixels with the Wu quantizer."""
    if isinstance(pixels, Buffer):
        return _quantize_wu_raw(
            pixels,
            max_colors,
            pixel_format,
            alpha_threshold,
            premultiplied,
            width,
            stride,
        )

    if isinstance(pixels, Sequence):
        return _quantize_wu_rgb(pixels, max_colors)

    if pixels.mode in ("RGB", "RGBX"):
        return _quantize_wu_raw(
            pixels.tobytes("raw", "rgb"),
            max_colors,
        )
    elif pixels.mode == "RGBA":
        return _quantize_wu_raw(
            pixels.tobytes(),
            max_colors,
            PixelFormat.Rgba,
            alpha_threshold,
        )
    else:
        raise NotImplementedError(f"Unsupported mode: {pixels.mode}")

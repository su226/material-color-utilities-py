from collections.abc import Buffer
from typing import Never, overload

try:
    from PIL.Image import Image
except ImportError:
    Image = Never

from material_color_utilities._core import SRGB_TO_XYZ as SRGB_TO_XYZ
from material_color_utilities._core import WHITE_POINT_D65 as WHITE_POINT_D65
from material_color_utilities._core import XYZ_TO_SRGB as XYZ_TO_SRGB
from material_color_utilities._core import Cam16 as Cam16
from material_color_utilities._core import CorePalette as CorePalette
from material_color_utilities._core import CorePalettes as CorePalettes
from material_color_utilities._core import Hct as Hct
from material_color_utilities._core import Palettes as Palettes
from material_color_utilities._core import Platform as Platform
from material_color_utilities._core import Random as Random
from material_color_utilities._core import Scheme as Scheme
from material_color_utilities._core import Schemes as Schemes
from material_color_utilities._core import SpecVersion as SpecVersion
from material_color_utilities._core import Theme as Theme
from material_color_utilities._core import ThemeBuilder as ThemeBuilder
from material_color_utilities._core import TonalPalette as TonalPalette
from material_color_utilities._core import Variant as Variant
from material_color_utilities._core import ViewingConditions as ViewingConditions
from material_color_utilities._core import _extract_color_pil, _extract_color_raw
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
def extract_color(image: Buffer) -> RGB: ...


@overload
def extract_color(image: Image, alpha_threshold: int = 128) -> RGB: ...


def extract_color(
    image: Image | Buffer,
    alpha_threshold: int = 128,
) -> RGB:
    """
    Extract primary color from an image.

    :param image: Pillow image in RGB, RGBX or RGBA mode.
                  Or bytes object of raw 3 * n bytes data in RGBRGBRGB order.
    :param alpha_threshold: If Pillow image with mode of RGBA is used,
                            cut off alpha below the threshold.
                            Warning: slower if threshold is not 0.
    :return: 3-tuple of (R, G, B)
    """
    if isinstance(image, Buffer):
        return _extract_color_raw(image)
    if image.mode in ("RGB", "RGBX"):
        return _extract_color_pil(image, 0)
    elif image.mode == "RGBA":
        return _extract_color_pil(image, alpha_threshold)
    else:
        raise NotImplementedError(f"Unsupported mode: {image.mode}")

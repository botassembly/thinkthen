"""Generate the schema-owned relation type when assembling a Python package."""

from pathlib import Path

import maturin
from maturin import (
    get_requires_for_build_editable,
    get_requires_for_build_sdist,
    get_requires_for_build_wheel,
)

from relation_type import relation_type


def build_wheel(wheel_directory, config_settings=None, metadata_directory=None):
    relation_type(Path(__file__).parent / "thinkthen")
    return maturin.build_wheel(wheel_directory, config_settings, metadata_directory)


def build_editable(wheel_directory, config_settings=None, metadata_directory=None):
    relation_type(Path(__file__).parent / "thinkthen")
    return maturin.build_editable(wheel_directory, config_settings, metadata_directory)


def build_sdist(sdist_directory, config_settings=None):
    relation_type(Path(__file__).parent / "thinkthen")
    return maturin.build_sdist(sdist_directory, config_settings)


__all__ = [
    "build_editable",
    "build_sdist",
    "build_wheel",
    "get_requires_for_build_editable",
    "get_requires_for_build_sdist",
    "get_requires_for_build_wheel",
]

if __name__ == "__main__":
    relation_type(Path(__file__).parent / "thinkthen")

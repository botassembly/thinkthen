"""Report native module load failures before importing the public facade."""

from importlib import import_module as _import_module
from importlib.machinery import EXTENSION_SUFFIXES as _EXTENSION_SUFFIXES
from pathlib import Path as _Path

try:
    _import_module("._native", __package__)
except ImportError as _load_error:
    _paths = ", ".join(
        str(_Path(__file__).with_name("_native" + _suffix))
        for _suffix in _EXTENSION_SUFFIXES
    )
    raise ImportError(
        f"cannot load native engine; attempted {_paths}: {_load_error}"
    ) from _load_error

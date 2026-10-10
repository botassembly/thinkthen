import json

import pathlib

import sys

import pytest

from conftest import Backend, clean_env, private_windows_configuration, run

ROWS = json.loads((pathlib.Path(__file__).resolve().parents[3] / "conformance/binding-backends.json").read_text())["backends"]

SLOTS = ("generic_systemone", "generic_decisions", "generic_custom", "capture_systemone", "capture_decisions", "capture_custom", "other", "non_post")

def paths(slot, count=1):
    return {**dict.fromkeys(SLOTS, 0), slot: count, "overflow": False}

def isolated(folder, **extra):
    return clean_env(home=folder, **extra)

def configuration(folder, value):
    directory = folder / ("Library/Application Support/thinkthen" if sys.platform == "darwin" else "config/thinkthen")
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "config.json").write_text(json.dumps({"schema": "thinkthen.config/1", **value}))
    if sys.platform == "win32":
        private_windows_configuration(directory / "config.json")


def _answer(question):
    if question["type"] == "noul":
        return {"type": "noul", "noul": 0.9}
    if question["type"] == "choice":
        names = list(question["criteria"])
        return {"type": "choice", "probabilities": {
            name: float(index == 0) for index, name in enumerate(names)}}
    names = [str(index) for index in range(len(question["criteria"]))]
    return {"type": "score", "probabilities": {
        name: float(index == 0) for index, name in enumerate(names)}}


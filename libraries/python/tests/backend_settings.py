import json

import sys
from conftest import clean_env, private_windows_configuration

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

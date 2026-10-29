"""Run every function through installed pandas and Polars frame accessors."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from typing import Any

import pandas as pd
import polars as pl
from conformance import Loopback, admitted

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars


def check_frames(cases: Path) -> None:
    manifest = json.loads((cases / "typed/manifest.json").read_text())
    selected: dict[str, str] = {}
    for filename in manifest["cases"]:
        case = json.loads((cases / "typed" / filename).read_text())
        selected.setdefault(case["request"]["question"]["function"], filename)
    for wire in manifest["backends"]:
        for function, filename in selected.items():
            path = cases / "typed" / filename
            case = json.loads(path.read_text())
            definition = dict(case["request"]["question"])
            records = case["request"]["records"]
            expected = case["expected"]
            # The frame surface owns column selection and alignment, while the
            # complete typed rows are checked once by the base SDK driver.
            if function == "rank":
                records, expected = records[:2], expected[:1]
            if function == "filter":
                records, expected = records[:3], expected[:3]
            if function == "annotate":
                question: Any = tt.AnnotationQuestions(
                    {"ready": tt.DecideQuestion("Ready?")}
                )
                evidence = "/record/body"
            else:
                definition["evidence"] = "/record"
                question = admitted(definition)
                evidence = None
            for frame_name in ("pandas", "polars"):
                frame: Any = (
                    pd.DataFrame({"record": records})
                    if frame_name == "pandas"
                    else pl.DataFrame({"record": records}, strict=False)
                )
                with (
                    Loopback(path) as server,
                    tt.Client(
                        backend=wire,
                        url=server.url,
                        model=manifest["client"]["model"],
                        api_key="synthetic-loopback-key",
                        batch=case.get("client", {}).get("batch", 32),
                        retry=tt.RetryPolicy(max_retries=0),
                    ) as client,
                ):
                    options = {"client": client, "name": "frame-" + function}
                    if evidence is not None:
                        options["evidence"] = evidence
                    result = getattr(frame.thinkthen, function)(question, **options)
                    assert len(result) == len(expected), (wire, frame_name, function)
                    if function == "annotate":
                        statuses = (
                            result["ready_status"].tolist()
                            if frame_name == "pandas"
                            else result.get_column("ready_status").to_list()
                        )
                        assert statuses == ["answered"], (
                            frame_name,
                            function,
                            statuses,
                        )
                    elif frame_name == "pandas":
                        assert result["position"].tolist() == [
                            row["position"] for row in expected
                        ]
                        assert result["status"].tolist() == [
                            row["status"] for row in expected
                        ]
                        assert result["function"].tolist() == [function] * len(expected)
                        assert all(
                            source.name == "frame-" + function
                            for source in result["source"]
                        )
                    else:
                        rows = result.get_column("result").to_list()
                        assert [row["position"] for row in rows] == [
                            row["position"] for row in expected
                        ]
                        assert [row["status"] for row in rows] == [
                            row["status"] for row in expected
                        ]
                        assert [row["function"] for row in rows] == [function] * len(
                            expected
                        )
                        assert all(
                            row["source"]["name"] == "frame-" + function for row in rows
                        )
                    assert server.count() == case["requests"], (
                        wire,
                        frame_name,
                        function,
                    )
                print(f"Frames: {wire} {frame_name} {function}: PASS", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--cases",
        type=Path,
        default=Path(os.environ.get("THINKTHEN_CASES", "conformance")),
    )
    check_frames(parser.parse_args().cases)

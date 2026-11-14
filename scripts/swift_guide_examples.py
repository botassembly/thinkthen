"""Dispatch the Swift ideal guide through its already-built external consumer."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parent.parent


def run_swift(consumer, negative=None):
    if not consumer:
        raise RuntimeError('Swift guide requires --swift-consumer with its prepared package')
    folder = Path(consumer).resolve()
    if not (folder / '.build/debug/LiteralGuide').is_file():
        raise RuntimeError('Swift guide consumer must be built before scenario execution')
    runner = runpy.run_path(str(ROOT / 'sdk/swift/tests/ideal.py'))['run_swift']
    runner(folder, negative)

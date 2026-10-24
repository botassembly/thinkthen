"""Keep large release tool homes out of temporary storage."""
from pathlib import Path
import tempfile


def checked_home():
    """Resolve the active host's HOME before release preparation or building."""
    home = Path.home().resolve()
    temporary_roots = (Path('/tmp'), Path('/private/tmp'), Path(tempfile.gettempdir()))
    if any(home.is_relative_to(root.resolve()) for root in temporary_roots):
        job_home = Path(__file__).resolve().parent.parent / '.release-home'
        raise ValueError(f'release HOME resolves to temporary storage: {home}; '
                         'set HOME to a job-owned directory on disk-backed storage '
                         f'(for example, {job_home}) before preparing or building')
    return home

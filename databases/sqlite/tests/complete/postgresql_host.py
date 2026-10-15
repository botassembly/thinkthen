"""Restart an owned installed PostgreSQL server with each case's isolated environment."""
import os,subprocess,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[4]/'conformance/children'))
from children import child_env
class PostgresqlHost:
    def __init__(self):
        self.bin=Path(os.environ['THINKTHEN_POSTGRESQL_BIN'])
        self.data=Path(os.environ['THINKTHEN_POSTGRESQL_DATA'])
        self.socket=os.environ['THINKTHEN_POSTGRESQL_SOCKET']
        self.log=self.data.parent/'complete-server.log'
    def stop(self):
        if (self.data/'postmaster.pid').exists():
            subprocess.run([str(self.bin/'pg_ctl'),'-D',str(self.data),'-m','immediate','-w','stop'],env=child_env(keep=('LANG','LC_ALL'),HOME=str(self.data.parent)),capture_output=True,check=True,timeout=30)
    def start(self,env):
        self.stop()
        try:
            subprocess.run([str(self.bin/'pg_ctl'),'-D',str(self.data),'-l',str(self.log),'-w','start'],env=env,capture_output=True,check=True,timeout=30)
        except subprocess.CalledProcessError as error:
            log=self.log.read_text()[-4000:] if self.log.is_file() else ''
            raise AssertionError((error.returncode,error.stdout,error.stderr,log)) from error
        subprocess.run(['psql','-X','-q','-v','ON_ERROR_STOP=1','-h',self.socket,'-U','postgres','-d','postgres','-c','CREATE EXTENSION IF NOT EXISTS thinkthen'],env=env,capture_output=True,check=True,timeout=30)

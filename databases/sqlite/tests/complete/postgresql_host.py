"""Restart an owned installed PostgreSQL server with each case's isolated environment."""
import os,subprocess
from pathlib import Path
class PostgresqlHost:
    def __init__(self):
        self.bin=Path(os.environ['THINKTHEN_POSTGRESQL_BIN'])
        self.data=Path(os.environ['THINKTHEN_POSTGRESQL_DATA'])
        self.socket=os.environ['THINKTHEN_POSTGRESQL_SOCKET']
        self.log=self.data.parent/'complete-server.log'
    def stop(self):
        if (self.data/'postmaster.pid').exists():
            subprocess.run([str(self.bin/'pg_ctl'),'-D',str(self.data),'-m','immediate','-w','stop'],capture_output=True,check=True,timeout=30)
    def start(self,env):
        self.stop()
        subprocess.run([str(self.bin/'pg_ctl'),'-D',str(self.data),'-l',str(self.log),'-w','start'],env=env,capture_output=True,check=True,timeout=30)
        subprocess.run(['psql','-X','-q','-v','ON_ERROR_STOP=1','-h',self.socket,'-U','postgres','-d','postgres','-c','CREATE EXTENSION IF NOT EXISTS thinkthen'],env=env,capture_output=True,check=True,timeout=30)

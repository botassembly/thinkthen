#!/usr/bin/env python3
"""Windows release tools enter an owned kill-on-close job before their first instruction."""
import ctypes
from ctypes import wintypes as W
import time


class Limits(ctypes.Structure):
    _fields_ = [('PerProcessUserTimeLimit', ctypes.c_longlong), ('PerJobUserTimeLimit', ctypes.c_longlong),
        ('LimitFlags', W.DWORD), ('MinimumWorkingSetSize', ctypes.c_size_t),
        ('MaximumWorkingSetSize', ctypes.c_size_t), ('ActiveProcessLimit', W.DWORD),
        ('Affinity', ctypes.c_size_t), ('PriorityClass', W.DWORD), ('SchedulingClass', W.DWORD)]


class IO(ctypes.Structure):
    _fields_ = [(name, ctypes.c_ulonglong) for name in
        ('ReadOperationCount', 'WriteOperationCount', 'OtherOperationCount',
         'ReadTransferCount', 'WriteTransferCount', 'OtherTransferCount')]


class Extended(ctypes.Structure):
    _fields_ = [('BasicLimitInformation', Limits), ('IoInfo', IO),
        ('ProcessMemoryLimit', ctypes.c_size_t), ('JobMemoryLimit', ctypes.c_size_t),
        ('PeakProcessMemoryUsed', ctypes.c_size_t), ('PeakJobMemoryUsed', ctypes.c_size_t)]


class ThreadEntry(ctypes.Structure):
    _fields_ = [('dwSize', W.DWORD), ('cntUsage', W.DWORD), ('th32ThreadID', W.DWORD),
        ('th32OwnerProcessID', W.DWORD), ('tpBasePri', W.LONG), ('tpDeltaPri', W.LONG), ('dwFlags', W.DWORD)]


class Accounting(ctypes.Structure):
    _fields_ = [(name, ctypes.c_longlong) for name in
        ('TotalUserTime', 'TotalKernelTime', 'ThisPeriodTotalUserTime', 'ThisPeriodTotalKernelTime')] + [
        (name, W.DWORD) for name in ('TotalPageFaultCount', 'TotalProcesses', 'ActiveProcesses', 'TotalTerminatedProcesses')]


def api():
    kernel = ctypes.WinDLL('kernel32', use_last_error=True)
    signatures = {
        'CreateJobObjectW': ([ctypes.c_void_p, W.LPCWSTR], W.HANDLE),
        'SetInformationJobObject': ([W.HANDLE, ctypes.c_int, ctypes.c_void_p, W.DWORD], W.BOOL),
        'AssignProcessToJobObject': ([W.HANDLE, W.HANDLE], W.BOOL),
        'TerminateJobObject': ([W.HANDLE, W.UINT], W.BOOL),
        'QueryInformationJobObject': ([W.HANDLE, ctypes.c_int, ctypes.c_void_p, W.DWORD, ctypes.c_void_p], W.BOOL),
        'CloseHandle': ([W.HANDLE], W.BOOL),
        'CreateToolhelp32Snapshot': ([W.DWORD, W.DWORD], W.HANDLE),
        'Thread32First': ([W.HANDLE, ctypes.POINTER(ThreadEntry)], W.BOOL),
        'Thread32Next': ([W.HANDLE, ctypes.POINTER(ThreadEntry)], W.BOOL),
        'OpenThread': ([W.DWORD, W.BOOL, W.DWORD], W.HANDLE),
        'GetProcessIdOfThread': ([W.HANDLE], W.DWORD),
        'ResumeThread': ([W.HANDLE], W.DWORD),
    }
    for name, (arguments, result) in signatures.items():
        function = getattr(kernel, name)
        function.argtypes, function.restype = arguments, result
    return kernel


def checked(value):
    if not value:
        raise ctypes.WinError(ctypes.get_last_error())
    return value


class Job:
    def __init__(self):
        self.kernel = api()
        self.handle = checked(self.kernel.CreateJobObjectW(None, None))
        limits = Extended()
        limits.BasicLimitInformation.LimitFlags = 0x2000  # JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE; no breakaway.
        try:
            checked(self.kernel.SetInformationJobObject(self.handle, 9, ctypes.byref(limits), ctypes.sizeof(limits)))
        except BaseException:
            self.close()
            raise

    def start(self, child):
        # CPython Popen owns the actual process HANDLE, retained until child destruction.
        # No PID lookup or process-name selection determines job membership.
        checked(self.kernel.AssignProcessToJobObject(self.handle, int(child._handle)))
        # Popen closes the primary thread handle. Discover it while the root is suspended,
        # then validate the opened thread's owner before resuming its first instruction.
        snapshot = self.kernel.CreateToolhelp32Snapshot(4, 0)  # TH32CS_SNAPTHREAD
        if snapshot == ctypes.c_void_p(-1).value:
            raise ctypes.WinError(ctypes.get_last_error())
        try:
            entry = ThreadEntry()
            entry.dwSize = ctypes.sizeof(entry)
            present = self.kernel.Thread32First(snapshot, ctypes.byref(entry))
            while present:
                if entry.th32OwnerProcessID == child.pid:
                    thread = checked(self.kernel.OpenThread(0x0802, False, entry.th32ThreadID))
                    try:
                        if self.kernel.GetProcessIdOfThread(thread) != child.pid:
                            raise ValueError('owned suspended thread identity changed')
                        if self.kernel.ResumeThread(thread) != 1:
                            raise ValueError('owned primary thread did not resume from suspension')
                        return
                    finally:
                        checked(self.kernel.CloseHandle(thread))
                present = self.kernel.Thread32Next(snapshot, ctypes.byref(entry))
            raise ValueError('owned suspended primary thread is missing')
        finally:
            checked(self.kernel.CloseHandle(snapshot))

    def close(self):
        if self.handle:
            try:
                checked(self.kernel.TerminateJobObject(self.handle, 1))
                deadline = time.monotonic() + 5
                accounting = Accounting()
                while True:
                    checked(self.kernel.QueryInformationJobObject(self.handle, 1,
                        ctypes.byref(accounting), ctypes.sizeof(accounting), None))
                    if accounting.ActiveProcesses == 0:
                        break
                    if time.monotonic() >= deadline:
                        raise ValueError('owned job cleanup timed out')
                    time.sleep(0.02)
            finally:
                checked(self.kernel.CloseHandle(self.handle))
                self.handle = None

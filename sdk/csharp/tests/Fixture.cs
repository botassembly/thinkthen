using System.Diagnostics;

internal sealed class Fixture : IAsyncDisposable
{
    private readonly Process process;
    internal string Url { get; }
    private Fixture(Process process, string port)
    { this.process = process; Url = $"http://127.0.0.1:{int.Parse(port)}/v1"; }
    internal static async Task<Fixture> Start(string executable, params string[] arguments)
    {
        var options = new ProcessStartInfo(executable)
        { RedirectStandardInput = true, RedirectStandardOutput = true, UseShellExecute = false };
        foreach (string argument in arguments) options.ArgumentList.Add(argument);
        var process = Process.Start(options)!;
        try { return new(process, (await process.StandardOutput.ReadLineAsync().WaitAsync(TimeSpan.FromSeconds(10)))!); }
        catch { process.Kill(); process.Dispose(); throw; }
    }
    internal async Task<string> Command(string command)
    {
        await process.StandardInput.WriteLineAsync(command);
        await process.StandardInput.FlushAsync();
        return (await process.StandardOutput.ReadLineAsync().WaitAsync(TimeSpan.FromSeconds(command == "closed" ? 2 : 10)))!;
    }
    public async ValueTask DisposeAsync()
    {
        await process.StandardInput.WriteLineAsync("shutdown");
        await process.StandardInput.FlushAsync();
        try { await process.WaitForExitAsync().WaitAsync(TimeSpan.FromSeconds(10)); }
        catch { process.Kill(entireProcessTree: true); process.Dispose(); throw; }
        int exit = process.ExitCode;
        process.Dispose();
        if (exit != 0) throw new Exception("loopback fixture failed");
    }
}

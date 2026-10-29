import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public final class BackendFixture implements AutoCloseable {
    private final Process process;
    private final BufferedReader output;
    private final BufferedWriter input;
    private final String base;
    public BackendFixture(String mode) throws IOException {
        var args = new java.util.ArrayList<String>();
        args.add(System.getenv("THINKTHEN_LOOPBACK"));
        if (mode.equals("unavailable")) args.add("--unavailable");
        else if (java.util.List.of("headers", "body", "refusal-body").contains(mode)) { args.add("--held"); args.add(mode); }
        else args.add(mode);
        process = new ProcessBuilder(args).redirectError(ProcessBuilder.Redirect.INHERIT).start();
        output = new BufferedReader(new InputStreamReader(process.getInputStream(), StandardCharsets.UTF_8));
        input = new BufferedWriter(new OutputStreamWriter(process.getOutputStream(), StandardCharsets.UTF_8));
        String port = readLine(10);
        if (port == null) throw new IOException("Loopback fixture failed before publishing its port");
        base = "http://127.0.0.1:" + Integer.parseInt(port) + "/v1";
    }
    public String base() { return base; }
    public synchronized String command(String command) throws IOException {
        input.write(command); input.newLine(); input.flush();
        String result = readLine(command.equals("closed") ? 2 : 10);
        if (result == null) throw new IOException("Loopback fixture stopped during " + command);
        return result;
    }
    private String readLine(int seconds) throws IOException {
        var pending = new java.util.concurrent.FutureTask<String>(output::readLine);
        Thread.ofPlatform().daemon().name("loopback-pipe-reader").start(pending);
        try { return pending.get(seconds,java.util.concurrent.TimeUnit.SECONDS); }
        catch (java.util.concurrent.TimeoutException | java.util.concurrent.ExecutionException error) {
            process.destroyForcibly();
            throw new IOException("Loopback response did not arrive",error);
        } catch (InterruptedException error) {
            process.destroyForcibly(); Thread.currentThread().interrupt(); throw new IOException(error);
        }
    }
    @Override public void close() throws IOException {
        input.write("shutdown"); input.newLine(); input.flush(); input.close();
        try {
            if (!process.waitFor(10,java.util.concurrent.TimeUnit.SECONDS)) { process.destroyForcibly(); throw new IOException("Loopback did not stop"); }
            if (process.exitValue() != 0) throw new IOException("Loopback fixture failed");
        } catch (InterruptedException error) {
            Thread.currentThread().interrupt();
            throw new IOException(error);
        } finally { output.close(); process.destroy(); }
    }
}

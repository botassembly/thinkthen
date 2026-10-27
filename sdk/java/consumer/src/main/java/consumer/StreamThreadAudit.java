package consumer;

import java.nio.file.Files;
import jdk.jfr.Recording;
import jdk.jfr.consumer.RecordingFile;

/** Count actual thread starts, including threads that finish before observation. */
public final class StreamThreadAudit implements AutoCloseable {
    private final Recording recording = new Recording();
    public StreamThreadAudit() {
        recording.enable("jdk.ThreadStart");
        recording.start();
    }
    @Override public void close() throws java.io.IOException {
        recording.stop();
        var file = Files.createTempFile("stream-thread-audit-", ".jfr");
        try {
            recording.dump(file);
            long starts = RecordingFile.readAllEvents(file).stream()
                .filter(event -> event.getEventType().getName().equals("jdk.ThreadStart"))
                .filter(event -> event.getThread("thread").getJavaName().startsWith("thinkthen-"))
                .count();
            if (starts != 1) throw new AssertionError("Expected one driver thread per stream, started " + starts);
        } finally { recording.close(); Files.deleteIfExists(file); }
    }
}

import thinkthen.ResultEnvelope;
public final class ResultEnvelopeTest {
    private static void rejects(String result) {
        try { ResultEnvelope.value(result); }
        catch (AssertionError expected) { return; }
        throw new AssertionError("accepted malformed result envelope: " + result);
    }
    public static void main(String[] args) {
        String reversed = "{\"facts\":{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.1},\"value\":\"first\"}";
        if (!"\"first\"".equals(ResultEnvelope.value(reversed))) throw new AssertionError("valid reversed member order");
        rejects("{\"value\":\"first\",\"facts\":{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.1},\"extra\":1}");
        rejects("{\"value\":\"first\",\"facts\":{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":\"slow\"}}");
        rejects("{\"value\":\"first\",\"facts\":{\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.1}}");
        rejects("{\"value\":\"first\",\"facts\":{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":0.1,\"other\":4}}");
        System.out.println("REPIN_RESULT_ENVELOPE_NEGATIVES_PASS four cases");
    }
}

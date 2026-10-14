import thinkthen.*;
import java.math.*;
import java.util.*;
/** Preserve authored values and settle probabilities through installed calls. */
public final class ResultValuesCases {
    private static void check(boolean value, String message) { if (!value) throw new AssertionError(message); }
    public static void run(Engine engine) {
        var wide = new BigInteger("18446744073709551615");
        var object = new LinkedHashMap<String,Object>();
        object.put("wide", wide); object.put("flag", false); object.put("nil", null);
        object.put("nested", Arrays.asList(false, null, List.of("kept")));
        var readings = Arrays.<Object>asList(false, null, object, Arrays.asList(false, null, wide));
        for (int index = 0; index < readings.size(); index++) {
            Object authored = readings.get(index);
            var definition = new Inputs.RequestDefinitionAuthoredDecide().decide(new Inputs.AuthoredQuestionText("Is it?"));
            if (index == 0) definition.threshold(new Inputs.AuthoredThreshold(0.95));
            else if (authored == null) definition.trueValueNull();
            else if (index == 2) definition.trueValue(new Inputs.AuthoredCriterion(object));
            else definition.trueValue(new Inputs.AuthoredCriterion(Arrays.asList(false, null, wide)));
            var result = engine.decide(new Inputs.RequestQuestionDefinition().value(definition), new Inputs.RequestInputText().text("result-authored-" + index)).join();
            check(result.probability().compareTo(new BigDecimal("0.9")) == 0, "authored false or null remains resolved");
            if (index < 2) check(Objects.equals(result.value(), authored), "authored false and null survive");
            else if (index == 2) {
                var value = (Map<?,?>)result.value();
                check(value.get("wide").equals(new BigDecimal(wide)) && Boolean.FALSE.equals(value.get("flag")) && value.containsKey("nil") && value.get("nil") == null, "authored object keeps exact integer and null");
                check(((List<?>)value.get("nested")).get(1) == null, "nested authored null survives");
            } else check(((List<?>)result.value()).get(2).equals(new BigDecimal(wide)), "authored list keeps exact integer");
            check(result.results().size() == 1 && result.facts().requestsSent().intValueExact() == 1, "authored receipts and facts remain attached");
        }
        var keptNull = engine.filter(examples.SampleInputs.decide(), new Inputs.RequestInputJson().value(null)).join();
        check(keptNull.value().equals(Collections.singletonList(null)) && keptNull.probability().equals(List.of(new BigDecimal("0.9"))), "filter preserves a selected original null");
        var threshold = new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionAuthoredDecide().decide(new Inputs.AuthoredQuestionText("Is it?")).threshold(new Inputs.AuthoredThreshold("0.2:0.95")));
        var records = new Inputs.RequestInputRecords().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("result-first")), new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("no"))));
        var decisions = engine.decide(threshold, records, new Inputs.RequestOptions().batch(new Inputs.RequestBatch(1))).join();
        check(decisions.value().equals(Arrays.asList(null, false)), "list keeps unresolved and false in input order");
        check(decisions.probability().equals(Arrays.asList(null, new BigDecimal("0.1"))), "list probabilities follow normalized resolution and index");
        var choose = new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionAuthoredChoose().choose(new Inputs.AuthoredQuestionText("Which?")).options(new Inputs.AuthoredOptions(List.of("first", "second"))).threshold(new Inputs.AuthoredCut(0.95)));
        var unresolved = engine.choose(choose, new Inputs.RequestInputText().text("result-choose-unsure")).join();
        check(unresolved.value() == null && unresolved.probability() == null, "unresolved choose has no probability");
        var ranked = engine.rank(examples.SampleInputs.decide(), records, new Inputs.RequestOptions().batch(new Inputs.RequestBatch(1))).join();
        check(ranked.value().equals(List.of(BigInteger.ZERO, BigInteger.ONE)) && ranked.probability().equals(List.of(new BigDecimal("0.9"), new BigDecimal("0.1"))), "rank keeps original indexes and aligned yes odds");
        var foundNull = engine.find(examples.SampleInputs.find(), new Inputs.RequestInputRecords().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(null)), new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("other"))))).join();
        check(foundNull.value() == null && foundNull.probability().compareTo(new BigDecimal("0.9")) == 0, "selected original null retains picked odds");
    }
}

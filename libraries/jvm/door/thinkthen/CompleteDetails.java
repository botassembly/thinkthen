package thinkthen;
import java.util.List;
import thinkthen.Complete.*;
public final class CompleteDetails {
 private CompleteDetails() {}
 public enum DeclarationKind { ABSENT, STRING, OBJECT }
 public enum PropertyKind { STRING, NUMBER, BOOLEAN, STRING_LIST }
 public record InputProperty(String name, PropertyKind kind) {}
 public record InputDeclaration(DeclarationKind kind, List<InputProperty> properties, List<String> required) {}
 public record QuestionAuthor(OptionalValue<String> name, OptionalValue<Long> wordingVersion, InputDeclaration itemSchema, InputDeclaration contextSchema) {}
 public record ReportedUsage(Boolean present, OptionalValue<Long> inputTokens, OptionalValue<Long> outputTokens) {}
 public record SourceDetail(Origin origin, String answeredBy, OptionalValue<Long> batchSize) {}
 public record InputView(OptionalValue<Content> original, OptionalValue<Location> position, OptionalValue<List<ImageView>> images) {}
 public record Details(OptionalValue<Question> question, OptionalValue<Rule> threshold, OptionalValue<String> rawPick, ReportedUsage usage, List<SourceDetail> questionSources, List<ObservationIdentity> observations, List<InputView> inputs) {}
 public record SourceEntity(Entity entity, OptionalValue<Location> position) {}
 public record SourceEntityEdge(String relation, SourceEntity source, SourceEntity target, Double probability, Boolean either) {}
 public record SourceRecognition(Boolean present, List<SourceEntity> entities, OptionalValue<List<SourceEntityEdge>> relations) {}
 public record SourceEndpoint(Long ordinal, Endpoint endpoint, Content record, OptionalValue<Location> position) {}
 public record SourceEdge(String relation, SourceEndpoint source, SourceEndpoint target, Double probability, Boolean either) {}
 public record SourceRelations(Boolean present, List<SourceEdge> edges) {}
}

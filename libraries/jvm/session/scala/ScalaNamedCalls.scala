// Generated from the shared Rust call graph. Do not edit.
package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import thinkthen.Results
import java.util.concurrent.CompletableFuture
import scala.jdk.CollectionConverters.*
abstract class ScalaNamedCalls(protected val namedEngine: Engine) {
protected def run[A,B <: OwnedCall](native: CompletableFuture[A])(convert: A => B): Call[B]
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Option[Inputs.RequestOptions]): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, Option(native.value()), Option(native.probability()), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputText): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Option[Inputs.RequestOptions]): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, Option(native.value()), Option(native.probability()), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = run(namedEngine.decide(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Option[Inputs.RequestOptions]): Call[CallResult[Option[String],Option[java.math.BigDecimal],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, Option(native.value()), Option(native.probability()), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputText): Call[CallResult[Option[String],Option[java.math.BigDecimal],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions): Call[CallResult[Option[String],Option[java.math.BigDecimal],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Option[Inputs.RequestOptions]): Call[CallResult[Option[String],Option[java.math.BigDecimal],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, Option(native.value()), Option(native.probability()), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson): Call[CallResult[Option[String],Option[java.math.BigDecimal],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions): Call[CallResult[Option[String],Option[java.math.BigDecimal],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = run(namedEngine.choose(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions): Call[CallResult[List[Option[String]],List[Option[java.math.BigDecimal]],Results.AtomicNullableString]] = choose(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Option[Inputs.RequestOptions]): Call[CallResult[List[String],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputText): Call[CallResult[List[String],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions): Call[CallResult[List[String],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Option[Inputs.RequestOptions]): Call[CallResult[List[String],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson): Call[CallResult[List[String],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions): Call[CallResult[List[String],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Option[Inputs.RequestOptions]): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => item1)), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Option[Inputs.RequestOptions]): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => item1)), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Option[Inputs.RequestOptions]): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => item1)), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Option[Inputs.RequestOptions]): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => item1)), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Option[Inputs.RequestOptions]): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = run(namedEngine.tag(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => item1)), None, native.results().asScala.toList, native.facts())
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions): Call[CallResult[List[List[String]],Option[Nothing],Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Option[Inputs.RequestOptions]): Call[CallResult[java.math.BigDecimal,Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value(), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputText): Call[CallResult[java.math.BigDecimal,Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions): Call[CallResult[java.math.BigDecimal,Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Option[Inputs.RequestOptions]): Call[CallResult[java.math.BigDecimal,Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value(), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson): Call[CallResult[java.math.BigDecimal,Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions): Call[CallResult[java.math.BigDecimal,Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Option[Inputs.RequestOptions]): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Option[Inputs.RequestOptions]): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Option[Inputs.RequestOptions]): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Option[Inputs.RequestOptions]): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Option[Inputs.RequestOptions]): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = run(namedEngine.score(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions): Call[CallResult[List[java.math.BigDecimal],Option[Nothing],Results.AtomicDouble]] = score(question, input, Some(options))
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicBoolean]] = run(namedEngine.filter(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => Option(item0)), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicBoolean]] = filter(question, input, None)
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[List[Option[Any]],List[Option[java.math.BigDecimal]],Results.AtomicBoolean]] = filter(question, input, Some(options))
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[List[java.math.BigInteger],List[Option[java.math.BigDecimal]],Results.AtomicNonZeroUsize]] = run(namedEngine.rank(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[List[java.math.BigInteger],List[Option[java.math.BigDecimal]],Results.AtomicNonZeroUsize]] = rank(question, input, None)
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[List[java.math.BigInteger],List[Option[java.math.BigDecimal]],Results.AtomicNonZeroUsize]] = rank(question, input, Some(options))
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.Find]] = run(namedEngine.find(question, input, options.orNull)) { native =>
new CallResult(native, Option(native.value()), Option(native.probability()), native.results().asScala.toList, native.facts())
}
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.Find]] = find(question, input, None)
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[Option[Any],Option[java.math.BigDecimal],Results.Find]] = find(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Option[Inputs.RequestOptions]): Call[CallResult[Results.AnnotatedRow,Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value(), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputText): Call[CallResult[Results.AnnotatedRow,Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions): Call[CallResult[Results.AnnotatedRow,Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Option[Inputs.RequestOptions]): Call[CallResult[Results.AnnotatedRow,Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value(), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson): Call[CallResult[Results.AnnotatedRow,Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions): Call[CallResult[Results.AnnotatedRow,Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = run(namedEngine.annotate(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), None, native.results().asScala.toList, native.facts())
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions): Call[CallResult[List[Results.AnnotatedRow],Option[Nothing],Results.Annotation]] = annotate(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Option[Inputs.RequestOptions]): Call[CallResult[Results.Recognize,List[Option[java.math.BigDecimal]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value(), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputText): Call[CallResult[Results.Recognize,List[Option[java.math.BigDecimal]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions): Call[CallResult[Results.Recognize,List[Option[java.math.BigDecimal]],Results.Recognition]] = recognize(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Option[Inputs.RequestOptions]): Call[CallResult[Results.Recognize,List[Option[java.math.BigDecimal]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value(), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson): Call[CallResult[Results.Recognize,List[Option[java.math.BigDecimal]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions): Call[CallResult[Results.Recognize,List[Option[java.math.BigDecimal]],Results.Recognition]] = recognize(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => Option(item1))), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => Option(item1))), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => Option(item1))), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => Option(item1))), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = run(namedEngine.recognize(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => item0.asScala.toList.map(item1 => Option(item1))), native.results().asScala.toList, native.facts())
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions): Call[CallResult[List[Results.Recognize],List[List[Option[java.math.BigDecimal]]],Results.Recognition]] = recognize(question, input, Some(options))
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[List[Results.RelatedEntityEdge],List[Option[java.math.BigDecimal]],Results.Relation]] = run(namedEngine.relate(question, input, options.orNull)) { native =>
new CallResult(native, native.value().asScala.toList.map(item0 => item0), native.probability().asScala.toList.map(item0 => Option(item0)), native.results().asScala.toList, native.facts())
}
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[List[Results.RelatedEntityEdge],List[Option[java.math.BigDecimal]],Results.Relation]] = relate(question, input, None)
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[List[Results.RelatedEntityEdge],List[Option[java.math.BigDecimal]],Results.Relation]] = relate(question, input, Some(options))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[?,?,Results.AtomicDecideValue]] = input match {
case selected: Inputs.RequestInputText => decide(question, selected, options)
case selected: Inputs.RequestInputJson => decide(question, selected, options)
case selected: Inputs.RequestInputRecords => decide(question, selected, options)
case selected: Inputs.RequestInputUnits => decide(question, selected, options)
case selected: Inputs.RequestInputEntities => decide(question, selected, options)
case selected: Inputs.RequestInputSource => decide(question, selected, options)
case selected: Inputs.RequestInputFeed => decide(question, selected, options)
}
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[?,?,Results.AtomicDecideValue]] = decide(question, input, None)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[?,?,Results.AtomicDecideValue]] = decide(question, input, Some(options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[?,?,Results.AtomicNullableString]] = input match {
case selected: Inputs.RequestInputText => choose(question, selected, options)
case selected: Inputs.RequestInputJson => choose(question, selected, options)
case selected: Inputs.RequestInputRecords => choose(question, selected, options)
case selected: Inputs.RequestInputUnits => choose(question, selected, options)
case selected: Inputs.RequestInputEntities => choose(question, selected, options)
case selected: Inputs.RequestInputSource => choose(question, selected, options)
case selected: Inputs.RequestInputFeed => choose(question, selected, options)
}
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[?,?,Results.AtomicNullableString]] = choose(question, input, None)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[?,?,Results.AtomicNullableString]] = choose(question, input, Some(options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[?,?,Results.AtomicArrayOfString]] = input match {
case selected: Inputs.RequestInputText => tag(question, selected, options)
case selected: Inputs.RequestInputJson => tag(question, selected, options)
case selected: Inputs.RequestInputRecords => tag(question, selected, options)
case selected: Inputs.RequestInputUnits => tag(question, selected, options)
case selected: Inputs.RequestInputEntities => tag(question, selected, options)
case selected: Inputs.RequestInputSource => tag(question, selected, options)
case selected: Inputs.RequestInputFeed => tag(question, selected, options)
}
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[?,?,Results.AtomicArrayOfString]] = tag(question, input, None)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[?,?,Results.AtomicArrayOfString]] = tag(question, input, Some(options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[?,?,Results.AtomicDouble]] = input match {
case selected: Inputs.RequestInputText => score(question, selected, options)
case selected: Inputs.RequestInputJson => score(question, selected, options)
case selected: Inputs.RequestInputRecords => score(question, selected, options)
case selected: Inputs.RequestInputUnits => score(question, selected, options)
case selected: Inputs.RequestInputEntities => score(question, selected, options)
case selected: Inputs.RequestInputSource => score(question, selected, options)
case selected: Inputs.RequestInputFeed => score(question, selected, options)
}
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[?,?,Results.AtomicDouble]] = score(question, input, None)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[?,?,Results.AtomicDouble]] = score(question, input, Some(options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[?,?,Results.Annotation]] = input match {
case selected: Inputs.RequestInputText => annotate(question, selected, options)
case selected: Inputs.RequestInputJson => annotate(question, selected, options)
case selected: Inputs.RequestInputRecords => annotate(question, selected, options)
case selected: Inputs.RequestInputUnits => annotate(question, selected, options)
case selected: Inputs.RequestInputEntities => annotate(question, selected, options)
case selected: Inputs.RequestInputSource => annotate(question, selected, options)
case selected: Inputs.RequestInputFeed => annotate(question, selected, options)
}
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[?,?,Results.Annotation]] = annotate(question, input, None)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[?,?,Results.Annotation]] = annotate(question, input, Some(options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions]): Call[CallResult[?,?,Results.Recognition]] = input match {
case selected: Inputs.RequestInputText => recognize(question, selected, options)
case selected: Inputs.RequestInputJson => recognize(question, selected, options)
case selected: Inputs.RequestInputRecords => recognize(question, selected, options)
case selected: Inputs.RequestInputUnits => recognize(question, selected, options)
case selected: Inputs.RequestInputEntities => recognize(question, selected, options)
case selected: Inputs.RequestInputSource => recognize(question, selected, options)
case selected: Inputs.RequestInputFeed => recognize(question, selected, options)
}
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call[CallResult[?,?,Results.Recognition]] = recognize(question, input, None)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call[CallResult[?,?,Results.Recognition]] = recognize(question, input, Some(options))
}
class OwnedCall private[scala](private val native: Engine.OwnedCall) {
def packets(): List[Results.SessionPacket] = native.packets().asScala.toList
def terminal(): Results.SessionPacketTerminal = native.terminal()
}
final class CallResult[V,P,R] private[scala](native: Engine.OwnedCall, val value: V, val probability: P, val results: List[R], val facts: Results.Facts) extends OwnedCall(native)

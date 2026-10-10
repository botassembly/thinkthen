"""Native Kotlin and Scala views over the same Rust-derived result graph."""
import json
from csharp import shape, nullable, enum_values
from jvm import name, member


def render(definitions, language):
    kotlin = language == 'kotlin'
    def convert(raw, expression='value', depth=0):
        source = shape(raw)
        if '$ref' in source:
            typ = name(source['$ref'].removeprefix('#/$defs/'))
            return typ, f'{typ}.read({expression})'
        if source == {} or source is True or source.get('type') == 'null':
            return ('Any?' if kotlin else 'Any'), f'Values.freeze({expression})'
        if enum_values(source) or isinstance(source.get('const'), str):
            return 'String', f'{expression} as String' if kotlin else f'{expression}.asInstanceOf[String]'
        kind = source.get('type')
        if kind == 'array':
            var = f'item{depth}'
            typ, read = convert(source['items'], var, depth + 1)
            return (f'List<{typ}>', f'Values.list({expression}) {{ {var} -> {read} }}') if kotlin else (f'List[{typ}]', f'{expression}.asInstanceOf[java.util.List[Any]].asScala.toList.map({var} => {read})')
        if kind == 'object':
            typ, read = convert(source.get('additionalProperties', {}), f'item{depth}', depth + 1)
            return (f'Map<String, {typ}>', f'Values.map({expression}) {{ item{depth} -> {read} }}') if kotlin else (f'Map[String, {typ}]', f'Values.`object`({expression}).asScala.toMap.map {{ case (key, item{depth}) => key -> ({read}) }}')
        if kind == 'integer':
            return ('BigInteger', f'Values.integer({expression})') if kotlin else ('BigInt', f'BigInt(Values.integer({expression}))')
        if kind == 'number':
            return ('BigDecimal', f'{expression} as BigDecimal') if kotlin else ('BigDecimal', f'BigDecimal({expression}.asInstanceOf[java.math.BigDecimal])')
        if kind in ('boolean', 'string'):
            typ = {'boolean': 'Boolean', 'string': 'String'}[kind]
            return typ, f'{expression} as {typ}' if kotlin else f'{expression}.asInstanceOf[{typ}]'
        raise ValueError(f'Native JVM result conversion needs a typed shape: {source}')
    def condition(tag):
        mode, field, value = tag
        literal = lambda k, v: f'objectValue[{json.dumps(k)}] == {json.dumps(v)}' if kotlin else f'objectValue.get({json.dumps(k)}) == {json.dumps(v)}'
        has = lambda k: f'objectValue.containsKey({json.dumps(k)})'
        if mode == 'literal': return literal(field, value)
        if mode == 'literals': return ' && '.join(literal(k, v) for k, v in value.items())
        if mode == 'member': return has(field)
        return ' && '.join([has(k) for k in value['required']] + ['!' + has(k) for k in value['excluded']])
    lines = ['// Generated from the Rust result graph. Do not edit.', f'package thinkthen.{language}']
    if kotlin:
        lines += ['import thinkthen.Values', 'import thinkthen.Presence', 'import java.math.*', 'object Results {',
                  'abstract class View(value: Any?) {', 'val json: Any? = Values.freeze(value)',
                  'fun presence(member: String): Presence.State { val objectValue = Values.`object`(json); return if (!objectValue.containsKey(member)) Presence.State.MISSING else if (objectValue[member] == null) Presence.State.NULL else Presence.State.VALUE }',
                  'protected fun required(member: String): Any? { val objectValue = Values.`object`(json); check(objectValue.containsKey(member)) { "Missing native member: $member" }; return objectValue[member] }',
                  'protected fun optional(member: String): Any? = Values.`object`(json)[member]', '}']
    else:
        lines += ['import thinkthen.Values', 'import thinkthen.Presence', 'import scala.jdk.CollectionConverters.*', 'object Results {',
                  'abstract class View(value: Any) {', 'val json: Any = Values.freeze(value)',
                  'def presence(member: String): Presence.State = { val objectValue = Values.`object`(json); if (!objectValue.containsKey(member)) Presence.State.MISSING else if (objectValue.get(member) == null) Presence.State.NULL else Presence.State.VALUE }',
                  'protected def required(member: String): Any = { val objectValue = Values.`object`(json); require(objectValue.containsKey(member), "Missing native member: " + member); objectValue.get(member) }',
                  'protected def optional(member: String): Option[Any] = { val objectValue = Values.`object`(json); if (objectValue.containsKey(member)) Some(objectValue.get(member)) else None }', '}']
    parents = {child: name(key) for key, source in definitions.items() for child, _ in source.get('variants', [])}
    for key, raw in definitions.items():
        typ, source = name(key), shape(raw)
        if 'variants' in source or 'primitive_variants' in source:
            lines += [f'sealed interface {typ} {{ val json: Any? }}' if kotlin else f'sealed trait {typ} {{ def json: Any }}',
                      f'companion_placeholder' if kotlin else f'object {typ} {{']
            # Kotlin interfaces carry their factory in the companion.
            if kotlin: lines[-2:] = [f'sealed interface {typ} {{ val json: Any?', 'companion object {']
            lines.append(f'fun read(value: Any?): {typ} {{' if kotlin else f'def read(value: Any): {typ} = {{')
            if 'variants' in source:
                lines.append('val objectValue = Values.`object`(value)')
                for child, tag in source['variants']:
                    lines.append(f'if ({condition(tag)}) return {name(child)}.read(value)')
            else:
                tests = {'boolean':'Boolean', 'string':'String', 'number':'java.math.BigDecimal', 'integer':'java.math.BigDecimal', 'array':'List<*>' if kotlin else 'java.util.List[?]', 'object':'Map<*, *>' if kotlin else 'java.util.Map[?, ?]'}
                for kind in source['primitive_variants']:
                    test = 'value == null' if kind == 'null' else f'value is {tests[kind]}' if kotlin else f'value.isInstanceOf[{tests[kind]}]'
                    lines.append(f'if ({test}) return {typ}{name(kind)}.read(value)')
            lines += ['throw IllegalStateException("Unknown native result alternative")' if kotlin else 'throw new IllegalStateException("Unknown native result alternative")', '}', '}']
            if kotlin: lines.append('}')
            if 'primitive_variants' in source:
                for kind in source['primitive_variants']:
                    target, read = convert(source.get('primitive_schemas', {}).get(kind, {'type':kind}))
                    child = typ + name(kind)
                    if kotlin:
                        lines += [f'class {child}(override val json: Any?) : {typ} {{ val value: {target} get() = {read.replace("value", "json")}', f'companion object {{ fun read(value: Any?): {child} = {child}(Values.freeze(value)) }}', '}']
                    else:
                        lines += [f'final class {child}(val json: Any) extends {typ} {{ def value: {target} = {read.replace("value", "json")} }}', f'object {child} {{ def read(value: Any): {child} = new {child}(Values.freeze(value)) }}']
            continue
        parent = parents.get(key)
        lines.append(f'class {typ}(rawValue: Any?) : View(rawValue)' + (f', {parent}' if parent else '') + ' {' if kotlin else f'final class {typ}(rawValue: Any) extends View(rawValue)' + (f' with {parent}' if parent else '') + ' {')
        if 'properties' in source:
            for field, spec in source['properties'].items():
                target, decode = convert(spec)
                required = field in source.get('required', [])
                nil = nullable(spec, definitions)
                quoted, method = json.dumps(field), "`" + member(field) + "`"
                read = f'required({quoted})' if required else f'optional({quoted})'
                if kotlin:
                    read_decode = f'{read}.let {{ value -> {decode} }}' if required and not nil else f'{read}?.let {{ value -> {decode} }}'
                    lines.append(f'val {method}: {target}' + ('?' if (nil or not required) and not target.endswith('?') else '') + f' get() = {read_decode}')
                else:
                    if required:
                        expr = f'{{ val value = {read}; require(value != null, "Null native member: " + {quoted}); {decode} }}' if not nil else f'Option({read}).map(value => {decode})'
                    else:
                        expr = f'{read}.map(value => Option(value).map(value => {decode}))' if nil else f'{read}.flatMap(value => Option(value).map(value => {decode}))'
                    result_type = target
                    if nil: result_type = f'Option[{result_type}]'
                    if not required: result_type = f'Option[{result_type}]'
                    lines.append(f'def {method}: {result_type} = {expr}')
        else:
            target, read = convert(source, 'json')
            lines.append(f'val value: {target} get() = {read}' if kotlin else f'def value: {target} = {read}')
        lines.append('}')
        lines.append(f'companion_placeholder' if kotlin else f'object {typ} {{ def read(value: Any): {typ} = new {typ}(value) }}')
        if kotlin:
            lines[-2:] = [f'companion object {{ fun read(value: Any?): {typ} = {typ}(value) }}', '}']
    return '\n'.join(lines + ['}']) + '\n'

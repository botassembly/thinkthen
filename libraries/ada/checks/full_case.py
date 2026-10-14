"""Build one installed typed Ada caller and encode its runtime fixture inputs."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
from session_values import ROOT, definitions, matches, name, template
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env

spec = importlib.util.spec_from_file_location('shared_session_inputs', ROOT / 'conformance/children/session_cases.py')
inputs = importlib.util.module_from_spec(spec)
spec.loader.exec_module(inputs)


def shape(source):
    alternatives = source.get('anyOf', source.get('oneOf'))
    if isinstance(source.get('type'), list):
        alternatives = [{**source, 'type': kind} for kind in source['type']]
    return alternatives


class Reader:
    """Derive typed readers and their token encoder from the same request graph."""
    def __init__(self):
        self.functions = {}
        self.tokens = []

    def value(self, key, source, value=None, encode=False):
        if not isinstance(source, dict):
            source = {}
        if '$ref' in source:
            key = source['$ref'].removeprefix('#/$defs/')
            return self.value(key, definitions[key], value, encode)
        alternatives = shape(source)
        if alternatives and len(alternatives) == 1 and not template.enum_values(source):
            return self.value(key, alternatives[0], value, encode)
        typ = name(key)
        call = 'Read_' + typ
        if not encode and typ in self.functions:
            return call
        if not encode:
            self.functions[typ] = ''
        declarations, body = [], []
        def token(text):
            self.tokens.append(str(text))
        def child(child_key, child_source, child_value=None):
            return self.value(child_key, child_source, child_value, encode)
        variants = source.get('variants')
        alternatives = shape(source)
        values = template.enum_values(source)
        if values:
            if encode: token(values.index(value))
            else: body.append(f"return {typ}'Val (Integer'Value (Get_Line (Input)));")
        elif variants or alternatives and len(alternatives) > 1:
            branches = [(branch, definitions[branch], template.identifier(key + '_arm_' + branch.removeprefix(key + '_'))) for branch, _ in variants] if variants else [(key + '_value_' + str(i), branch, name(key + '_arm_' + str(i))) for i, branch in enumerate(alternatives)]
            if encode:
                index = next(i for i, (_, branch, _) in enumerate(branches) if matches(branch, value))
                token(index)
                child(branches[index][0], branches[index][1], value)
            else:
                body.append('case Integer\'Value (Get_Line (Input)) is')
                for i, (branch_key, branch, kind) in enumerate(branches):
                    body.append(f'when {i} => return (Kind => {kind}, V_{i} => {child(branch_key, branch)});')
                body.append('when others => raise Program_Error with "invalid fixture variant"; end case;')
        else:
            if alternatives:
                source = alternatives[0]
            if 'properties' in source:
                fields = []
                for member, field in source['properties'].items():
                    field_key = key + '_field_' + member
                    required = member in source.get('required', [])
                    if encode:
                        present = member in value
                        if not required:
                            token(present)
                        if required or present:
                            child(field_key, field, value.get(member, field.get('const')))
                    else:
                        reader = child(field_key, field)
                        variable = 'F_' + name(member)
                        if required:
                            declarations.append(f'{variable} : constant {name(field_key)} := {reader};')
                        else:
                            # Optional wrapper names are defined by the package generator.
                            optional = name(field_key) + '_Optional'
                            declarations.append(f'function Read_{variable} return {optional} is begin if Boolean\'Value (Get_Line (Input)) then return (Present => True, Value => {reader}); else return (Present => False); end if; end;')
                            declarations.append(f'{variable} : constant {optional} := Read_{variable};')
                        fields.append(name(member) + ' => ' + variable)
                body.append('return (' + (', '.join(fields) if fields else 'null record') + ');')
            elif source.get('type') in ('array', 'object'):
                mapping = source.get('type') == 'object'
                item_source = (source.get('additionalProperties') if isinstance(source.get('additionalProperties'), dict) else {}) if mapping else source.get('items', {})
                if encode:
                    token(len(value))
                    for item in value.items() if mapping else value:
                        if mapping:
                            self.text(item[0]); item = item[1]
                        child(key + '_element', item_source, item)
                else:
                    reader = child(key + '_element', item_source)
                    declarations.extend([f'Result : {typ};', 'Count : constant Natural := Natural\'Value (Get_Line (Input));'])
                    if mapping:
                        body.append(f"for Index in 1 .. Count loop declare Key : constant Unbounded_String := Read_Text; Item : constant {name(key + '_element')} := {reader}; begin Result.Append ({typ + '_Entry'}'(Key => Key, Value => Item)); end; end loop; return Result;")
                    else:
                        body.append(f'for Index in 1 .. Count loop Result.Append ({reader}); end loop; return Result;')
            elif 'const' in source or source.get('type') == 'null':
                body.append('return (null record);')
            elif source.get('type') == 'string':
                if encode: self.text(value)
                else: body.append(f'return {typ} (Read_Text);')
            elif source.get('type') in ('boolean', 'number', 'integer'):
                if encode: token(value)
                else: body.append(f'return {typ}\'Value (Get_Line (Input));')
            else:
                if encode: self.native_value(value)
                else: body.append(f'return {typ} (Read_Native_Value);')
        if not encode:
            self.functions[typ] = f'function {call} return {typ} is\n' + '\n'.join(declarations) + '\nbegin\n' + '\n'.join(body) + f'\nend {call};'
        return call

    def native_value(self, value):
        if value is None:
            self.tokens.append('Null_Value')
        elif isinstance(value, bool):
            self.tokens.extend(['Boolean_Value', str(value)])
        elif isinstance(value, (int, float)):
            self.tokens.append('Number_Value')
            self.text(json.dumps(value))
        elif isinstance(value, str):
            self.tokens.append('String_Value')
            self.text(value)
        else:
            mapping = isinstance(value, dict)
            self.tokens.extend(['Object_Value' if mapping else 'Array_Value', str(len(value))])
            for item in value.items() if mapping else value:
                if mapping:
                    self.text(item[0])
                    item = item[1]
                self.native_value(item)

    def text(self, value):
        path = self.home / f'authored-text-{len(self.tokens)}.bin'
        path.write_bytes(value.encode())
        self.tokens.append(str(path))


def build(package, home):
    reader = Reader()
    calls = []
    for verb in ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate'):
        key = 'RequestCall'
        function = reader.value(key, definitions[key])
        calls.append(f'{"if" if not calls else "elsif"} Verb = "{verb}" then Thinkthen.Native_Sessions.Start (Client, Owner, {function}, "{verb}");')
    feed = reader.value('RequestSessionDescriptor', definitions['RequestSessionDescriptor'])
    source = Path(__file__).with_name('full_case.adb').read_text()
    source = source.replace('with Ada.Exceptions;', 'with Ada.Exceptions;\nwith Ada.Command_Line;\nwith Thinkthen.Values;')
    source = source.replace('-- REQUEST', 'Input : Ada.Text_IO.File_Type;\nfunction Read_Text return Unbounded_String is begin return Fixture_Text (Get_Line (Input)); end;\n' + NATIVE_READER + '\n'.join('function Read_' + typ + ' return ' + typ + ';' for typ in reader.functions) + '\n' + '\n'.join(reader.functions.values()))
    source = source.replace('-- SETTINGS', 'Ada.Command_Line.Argument (2),')
    source = source.replace('Thinkthen.Sessions.Calls.CALL_NAME (Client, Owner, Request);', 'Ada.Text_IO.Open (Input, Ada.Text_IO.In_File, Ada.Command_Line.Argument (1));\ndeclare Verb : constant String := Get_Line (Input); begin\n' + '\n'.join(calls) + '\nelse raise Program_Error with "unknown fixture call"; end if; end;')
    source = source.replace('-- CANCEL', 'if Boolean\'Value (Get_Line (Input)) then Thinkthen.Native_Sessions.Cancel (Owner); end if;')
    source = source.replace('-- FEED', f'declare Count : constant Natural := Natural\'Value (Get_Line (Input)); begin for Index in 1 .. Count loop declare Item : constant T_RequestSessionDescriptor := {feed}; Status : Push_Status; begin loop Thinkthen.Native_Sessions.Push (Owner, Item, Status); exit when Status /= Full; Drain_One; end loop; end; end loop; end;')
    source = source.replace('if HELD_CANCEL then', "if Boolean'Value (Get_Line (Input)) then")
    path = home / 'thinkthen-sessions-parity_case.adb'
    path.write_text(source)
    (home / 'thinkthen-sessions-parity_case.ads').write_text('procedure Thinkthen.Sessions.Parity_Case;\n')
    objects = home / 'ada-objects'
    objects.mkdir(exist_ok=True)
    binary = home / 'ada-consumer'
    result = subprocess.run(['gnatmake', '-q', '-gnat2022', '-I' + str(home), '-I' + str(package / 'src'), str(path), '-D', str(objects), '-o', str(binary), '-largs', str(package / 'native/lib/libthinkthen.a'), '-ldl', '-lpthread', '-lm'], env=child_env(), text=True, capture_output=True, timeout=180)
    assert result.returncode == 0, result.stdout + result.stderr
    return binary


def fixture(step, home):
    descriptor = inputs.descriptor(step, home)
    verb = descriptor.pop('verb')
    cancel = descriptor.pop('cancel')
    held = descriptor.pop('held_cancel')
    feed = []
    if step.get('incremental') and descriptor['input']['kind'] != 'source':
        feed = descriptor['input']['items']
        descriptor['input'] = {'kind': 'feed', 'name': 'records'}
    reader = Reader()
    reader.home = home
    reader.tokens = [verb]
    key = 'RequestCall'
    reader.value(key, definitions[key], {'function': verb, **descriptor}, True)
    reader.tokens.extend([str(cancel), str(len(feed))])
    for item in feed:
        reader.value('RequestSessionDescriptor', definitions['RequestSessionDescriptor'], {'item': item}, True)
    reader.tokens.append(str(held))
    path = home / 'ada-input.txt'
    path.write_text('\n'.join(reader.tokens) + '\n')
    return path


NATIVE_READER = """
function Read_Native_Value return Thinkthen.Values.Value is
   use Thinkthen.Values;
   Kind : constant Value_Kind := Value_Kind'Value (Get_Line (Input));
begin
   case Kind is
      when Null_Value => return Null_Item;
      when Boolean_Value => return Boolean_Item (Boolean'Value (Get_Line (Input)));
      when Number_Value => return Number_Item (To_String (Read_Text));
      when String_Value => return String_Item (To_String (Read_Text));
      when Array_Value =>
         declare
            Count : constant Natural := Natural'Value (Get_Line (Input));
            Items : Value_List (1 .. Count);
         begin
            for Item of Items loop Item := Read_Native_Value; end loop;
            return Array_Item (Items);
         end;
      when Object_Value =>
         declare
            Count : constant Natural := Natural'Value (Get_Line (Input));
            Items : Field_List (1 .. Count);
         begin
            for Item of Items loop
               declare Key : constant String := To_String (Read_Text); begin
                  Item := Named (Key, Read_Native_Value);
               end;
            end loop;
            return Object_Item (Items);
         end;
   end case;
end Read_Native_Value;
"""

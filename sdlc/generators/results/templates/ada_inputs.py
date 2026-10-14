"""Generate friendly private Ada inputs and their internal carrier projections."""
import re
from templates.ada import identifier, enum_values, ada_string


def friendly(raw):
    if raw == 'T_Authored_inputProperty_array_field_items': return 'Array_Item_Type'
    semantic = {'T_Authored_options':'Choice_Options', 'T_Authored_questionText':'Question_Wording', 'T_Authored_threshold':'Probability_Band', 'T_Authored_labels':'Tag_Labels', 'T_Authored_levels':'Score_Levels'}
    if raw in semantic: return semantic[raw]
    if raw == 'T_RecognizeQuestion': return 'Recognition_Rules'
    if raw == 'T_RelateQuestion': return 'Relation_Rules'
    if raw == 'T_RequestSessionDescriptor': return 'Feed_Item'
    if raw == 'T_RequestReaderFailure': return 'Reader_Error'
    limits = {'T_EngineSettings_field_max_estimated_input_tokens_total':'Input_Token_Limit', 'T_EngineSettings_field_max_requests':'Request_Limit', 'T_EngineSettings_field_max_requests_total':'Total_Request_Limit'}
    if raw in limits: return limits[raw]
    value = raw.removeprefix('T_').replace('Authored_', '').replace('Request', '')
    value = value.replace('_field_', '_').replace('_value_', '_')
    value = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', value)
    return {'JSON':'Value', 'Call':'Call', 'Engine_Settings':'Engine_Settings', 'Function':'Function_Kind'}.get(value, value)


def outputs(definitions, names, schemas, metadata=None, plan_definitions=None):
    representatives = {}
    ordered = sorted(names, key=lambda key: (identifier(key) != names[key], key))
    for key in ordered:
        raw = names[key]
        if raw in ('T_Request', 'T_RequestCall'):
            continue
        if raw not in representatives and key in schemas and '$ref' not in schemas[key]:
            source = schemas[key]
            alternatives = source.get('anyOf', source.get('oneOf', [])) if isinstance(source,dict) else []
            if len(alternatives) == 1:
                source = alternatives[0]
            representatives[raw] = (key, source)
    public, private, bodies, bridges, bridge_bodies = [], [], [], [], []
    public_names = {raw:friendly(raw) for raw in representatives}
    scalar_kinds = {}
    def actual(source):
        while isinstance(source,dict) and '$ref' in source:
            source = definitions[source['$ref'].removeprefix('#/$defs/')]
        return source if isinstance(source,dict) else {}
    def raw_child(key, field, source):
        return names[source['$ref'].removeprefix('#/$defs/')] if isinstance(source,dict) and '$ref' in source else names[key + '_field_' + field]
    def scalar(source):
        source = actual(source)
        return not any(member in source for member in ('properties','variants','anyOf','oneOf')) and source.get('type') not in ('array','object') and not isinstance(source.get('type'),list) and not enum_values(source)
    for raw,(key,source) in representatives.items():
        if scalar(source):
            kind = source.get('type')
            scalar_kinds[raw] = kind
            public_names[raw] = {'string':'Text','integer':('Unsigned' if source.get('format','').startswith('uint') else 'Integer_Value'),'number':'Number','boolean':'Boolean_Value','null':'Null_Item'}.get(kind,'Value')
            if 'const' in source or kind == 'null':
                public_names[raw] = None
    public += ['subtype Text is Ada.Strings.Unbounded.Unbounded_String;', 'subtype Unsigned is Interfaces.Unsigned_64;', 'subtype Integer_Value is Interfaces.Integer_64;', 'subtype Number is Long_Float;', 'subtype Boolean_Value is Boolean;', 'subtype Value is Thinkthen.Values.Value;']
    def argument(raw):
        return 'String' if scalar_kinds.get(raw) == 'string' else 'Thinkthen.Inputs.' + public_names[raw]
    def unwrap(raw, value):
        if public_names[raw] is None:
            return 'Requests.' + raw + "'(null record)"
        if scalar_kinds.get(raw) == 'string':
            return 'Requests.' + raw + ' (Ada.Strings.Unbounded.To_Unbounded_String (' + value + '))'
        return 'Native.Unwrap (' + value + ')'
    def order(raw):
        key, source = representatives[raw]
        if source.get('type') in ('array','object') and not any(k in source for k in ('properties','variants')):
            child = names[key + '_element']
            return (3, 1 + order(child)[1] if child in representatives else 1, raw)
        return (0 if raw in scalar_kinds else 1 if enum_values(source) else 2, 0, raw)
    for raw in sorted(representatives, key=order):
        key, source = representatives[raw]
        label = public_names[raw]
        if label is None:
            continue
        values = enum_values(source)
        if values:
            literals = [label + '_' + re.sub(r'[^a-zA-Z0-9]', '_', value.title()) for value in values]
            public.append('type ' + label + ' is (' + ', '.join(literals) + ');')
        elif source.get('type') in ('array','object') and not any(k in source for k in ('properties','variants')):
            mapping = source['type'] == 'object'
            child_raw = names[key + '_element']
            child = public_names[child_raw]
            if mapping:
                public.append(f'type {label}_Entry is record\nName : Text;\nItem : Thinkthen.Inputs.{child};\nend record;')
                child = label + '_Entry'
            public.append(f'type {label} is array (Positive range <>) of {child};')
        elif raw not in scalar_kinds:
            public.append('type ' + label + ' is private;')
            private.append('type ' + label + ' is record\nData : Requests.' + raw + ';\nend record;')
        conversion = f'function Unwrap (Item : Thinkthen.Inputs.{label}) return Requests.{raw}'
        projection = f'function Wrap (Item : Requests.{raw}) return Thinkthen.Inputs.{label}'
        bridges += [conversion+';', projection+';']
        if values:
            conversion_body = 'is (Requests.' + raw + "'Val (" + label + "'Pos (Item)));"
            projection_body = 'is (Thinkthen.Inputs.' + label + "'Val (Requests." + raw + "'Pos (Item)));"
        elif raw in scalar_kinds:
            conversion_body = 'is (Requests.' + raw + ' (Item));'
            projection_body = 'is (Thinkthen.Inputs.' + label + ' (Item));'
        elif source.get('type') in ('array','object') and not any(k in source for k in ('properties','variants')):
            mapping = source['type'] == 'object'
            conversion_body = f'is\nResult : Requests.{raw};\nbegin\nfor Child of Item loop\n' + ('Result.Append (Requests.'+raw+"_Entry'(Key => Child.Name, Value => Unwrap (Child.Item)));" if mapping else 'Result.Append (Unwrap (Child));') + '\nend loop;\nreturn Result;\nend Unwrap;'
            projection_body = f'is\nResult : {label} (1 .. Natural (Item.Length));\nPosition : Positive := 1;\nbegin\nfor Child of Item loop\nResult (Position) := ' + ('(Name => Child.Key, Item => Wrap (Child.Value));' if mapping else 'Wrap (Child);') + '\nPosition := Position + 1;\nend loop;\nreturn Result;\nend Wrap;'
        else:
            conversion_body = 'is (Item.Data);'
            projection_body = 'is ((Data => Item));'
        bridge_bodies += [conversion+' '+conversion_body, projection+' '+projection_body]
        if 'properties' in source:
            required, fields = [], []
            for member, child in source['properties'].items():
                child_raw = raw_child(key,member,child)
                if member in source.get('required',[]) and public_names[child_raw] is not None:
                    parameter = {'end':'Stop','type':'Category','record':'Record_Index','function':'Function_Name','at':'Position'}.get(member, member.title())
                    required.append(parameter + ' : ' + argument(child_raw))
                    fields.append(identifier(member) + ' => ' + unwrap(child_raw,parameter))
            signature = 'function Make_' + label + (' (' + '; '.join(required) + ')' if required else '') + ' return Thinkthen.Inputs.' + label
            public.append(signature + ';')
            bodies.append(signature + ' is ((Data => (' + ', '.join(fields + (['others => <>'] if len(fields) < len(source['properties']) else [])) + ')));')
            for member, child in source['properties'].items():
                child_raw = raw_child(key,member,child)
                if public_names[child_raw] is None:
                    continue
                optional = member not in source.get('required',[])
                field = 'Item.Data.' + identifier(member)
                signature = f'function With_{member.title()} (Item : Thinkthen.Inputs.{label}; Field : {argument(child_raw)}) return Thinkthen.Inputs.{label}'
                public.append(signature+';')
                assignment = unwrap(child_raw,'Field')
                if optional:
                    assignment = '(Present => True, Value => ' + assignment + ')'
                bodies.append(signature + ' is\nResult : Thinkthen.Inputs.' + label + ' := Item;\nbegin\nResult.Data.'+identifier(member)+' := '+assignment+';\nreturn Result;\nend With_'+member.title()+';')
                getter = f'function Get_{member.title()} (Item : Thinkthen.Inputs.{label}) return Thinkthen.Inputs.{public_names[child_raw]}'
                public.append(getter+';')
                check = f'if not {field}.Present then raise Constraint_Error with "missing input field"; end if;\n' if optional else ''
                bodies.append(getter+' is\nbegin\n'+check+'return Native.Wrap ('+field+('.Value' if optional else '')+');\nend Get_'+member.title()+';')
                if optional:
                    public.append(f'function Has_{member.title()} (Item : Thinkthen.Inputs.{label}) return Boolean;')
                    bodies.append(f'function Has_{member.title()} (Item : Thinkthen.Inputs.{label}) return Boolean is ({field}.Present);')
        alternatives = source.get('anyOf',source.get('oneOf',[]))
        if isinstance(source.get('type'),list): alternatives = [{**source,'type':kind} for kind in source['type']]
        variants = source.get('variants')
        branches = [(branch,{'$ref':'#/$defs/'+branch}) for branch,_ in variants] if variants else [(str(i),child) for i,child in enumerate(alternatives)]
        if branches and not values:
            kind_labels = [label + '_' + (friendly(names[child['$ref'].removeprefix('#/$defs/')]).removeprefix(label + '_') if '$ref' in child else str(actual(child).get('const', actual(child).get('type', 'Value'))).title()) for _,child in branches]
            kind_labels = [name + '_Case' for name in kind_labels]
            public.append('type ' + label + '_Kind is (' + ', '.join(kind_labels) + ');')
            public.append(f'function Kind (Item : Thinkthen.Inputs.{label}) return {label}_Kind;')
            bodies.append(f"function Kind (Item : Thinkthen.Inputs.{label}) return {label}_Kind is ({label}_Kind'Val (Requests.{raw}_Kind'Pos (Item.Data.Kind)));")
            for index,(tag,child) in enumerate(branches):
                host_key = child['$ref'].removeprefix('#/$defs/') if '$ref' in child else key+'_value_'+tag
                child_raw = names[host_key]
                child_schema = actual(child)
                child_label = public_names[child_raw]
                suffix = friendly(child_raw).removeprefix(label + '_') if variants else str(child_schema.get('const',child_schema.get('type','Value'))).title()
                suffix = {'Null':'Null_Item', 'Integer':'Integer_Value'}.get(suffix,suffix)
                signature = f'function {label}_From_{suffix}' + (f' (Item : {argument(child_raw)})' if child_label else '') + f' return Thinkthen.Inputs.{label}'
                public.append(signature+';')
                raw_tag = identifier(key + '_arm_' + tag.removeprefix(key+'_'))
                bodies.append(signature+f" is ((Data => (Kind => Requests.{raw_tag}, V_{index} => " + unwrap(child_raw,'Item')+')));')
                if label in ('Question','Input','Original') and 'properties' in child_schema:
                    required = [(member,field) for member,field in child_schema['properties'].items() if member in child_schema.get('required',[]) and public_names[raw_child(host_key,member,field)] is not None]
                    if len(required) == 1:
                        member, field = required[0]
                        value_raw = raw_child(host_key,member,field)
                        short = tag.removeprefix(key+'_').title() + '_' + label
                        short_signature = f'function {short} (Item : {argument(value_raw)}) return Thinkthen.Inputs.{label}'
                        public.append(short_signature+';')
                        value = '(' + identifier(member) + ' => ' + unwrap(value_raw,'Item') + ', others => <>)'
                        bodies.append(short_signature + f' is ((Data => (Kind => Requests.{raw_tag}, V_{index} => '+value+')));')
                if child_label:
                    getter = f'function As_{suffix} (Item : Thinkthen.Inputs.{label}) return Thinkthen.Inputs.{child_label}'
                    public.append(getter + ';')
                    bodies.append(getter + f' is (Native.Wrap (Item.Data.V_{index}));')
    public.append('function Text_Item (Item : String) return Thinkthen.Inputs.Item;')
    bodies.append('function Text_Item (Item : String) return Thinkthen.Inputs.Item is (Native.Wrap (Requests.Text_Item (Item)));')
    public += ['type Call_Options is private;', 'function No_Options return Call_Options;', 'function Some_Options (Item : Options) return Call_Options;', 'function Has_Options (Item : Call_Options) return Boolean;']
    private.append('type Call_Options is record\nData : Requests.T_RequestOptions_Optional;\nend record;')
    bodies += ['function No_Options return Call_Options is ((Data => (Present => False)));', 'function Some_Options (Item : Options) return Call_Options is ((Data => (Present => True, Value => Item.Data)));', 'function Has_Options (Item : Call_Options) return Boolean is (Item.Data.Present);']
    bridges += ['function Unwrap (Item : Call_Options) return Requests.T_RequestOptions_Optional;', 'function Wrap (Item : Requests.T_RequestOptions_Optional) return Call_Options;']
    bridge_bodies += ['function Unwrap (Item : Call_Options) return Requests.T_RequestOptions_Optional is (Item.Data);', 'function Wrap (Item : Requests.T_RequestOptions_Optional) return Call_Options is ((Data => Item));']
    for verb, raw in [('recognize','T_RecognizeQuestion'), ('relate','T_RelateQuestion')]:
        signature = f'function {verb.title()}_Question (Item : {public_names[raw]}) return Question'
        public.append(signature+';')
        bodies.append(signature+' is (Native.Wrap (Requests.'+verb.title()+'_Question (Native.Unwrap (Item))));')
    for verb in ('recognize','relate'):
        raw = next(names[key] for key,_ in definitions['RequestDefinition']['variants'] if verb in definitions[key].get('properties',{}))
        signature = f'function {verb.title()}_Question (Item : {public_names[raw]}) return Question'
        public.append(signature+';')
        bodies.append(signature+' is (Native.Wrap (Requests.'+verb.title()+'_Question (Native.Unwrap (Item))));')
    public = [line for line in public if not line.startswith('function ')] + [line for line in public if line.startswith('function ')]
    spec = '-- Generated friendly inputs; canonical carriers remain private.\nwith Ada.Strings.Unbounded;\nwith Interfaces;\nwith Thinkthen.Values;\nprivate with Thinkthen.Requests;\npackage Thinkthen.Inputs is\n'+'\n'.join(public)+'\nprivate\n'+'\n'.join(private)+'\nend Thinkthen.Inputs;\n'
    body = '-- Generated construction-only inputs; Rust owns semantic rules.\nwith Thinkthen.Inputs.Native;\npackage body Thinkthen.Inputs is\n'+'\n'.join(bodies)+'\nend Thinkthen.Inputs;\n'
    bridge_spec = '-- Generated internal carrier projections.\nwith Thinkthen.Requests;\nprivate package Thinkthen.Inputs.Native is\n'+'\n'.join(bridges)+'\nend Thinkthen.Inputs.Native;\n'
    bridge_body = '-- Generated internal carrier projections.\npackage body Thinkthen.Inputs.Native is\n'+'\n'.join(bridge_bodies)+'\nend Thinkthen.Inputs.Native;\n'
    if metadata is not None:
        metadata.update(public_names=public_names, scalar_kinds=scalar_kinds, representatives=representatives)
    call_spec = ['-- Generated named calls over friendly inputs.', 'with Thinkthen.Sessions;', 'package Thinkthen.Inputs.Calls is']
    call_body = ['-- Generated native admission; Rust owns all question semantics.', 'with Thinkthen.Inputs.Native;', 'with Thinkthen.Native_Sessions;', 'with Thinkthen.Requests; use Thinkthen.Requests;', 'package body Thinkthen.Inputs.Calls is']
    for function in enum_values(definitions['RequestFunction']):
        signature = f'procedure {function.title()} (Owner : in out Thinkthen.Sessions.Session; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options)'
        shared = signature.replace('(Owner', '(Client : Engine; Owner')
        call_spec += [signature+';',shared+';']
        for declaration,owner in ((signature,'Owner'),(shared,'Client, Owner')):
            call_body.append(declaration+' is\nbegin\nThinkthen.Native_Sessions.Start ('+owner+', (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), "'+function+'");\nend '+function.title()+';')
    call_spec += ['procedure Configure (Client : in out Engine; Settings : Engine_Settings; Error : out Failure);',
                  'procedure Push (Owner : in out Thinkthen.Sessions.Session; Item : Feed_Item; Status : out Thinkthen.Sessions.Push_Status);',
                  'procedure Finish (Owner : in out Thinkthen.Sessions.Session; Error : Reader_Error);',
                  'function Plan (Client : Engine; Function_Name : Function_Kind; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) return Plan_Estimate;']
    call_body += ['procedure Push (Owner : in out Thinkthen.Sessions.Session; Item : Feed_Item; Status : out Thinkthen.Sessions.Push_Status) is\nbegin\nThinkthen.Native_Sessions.Push (Owner, Native.Unwrap (Item), Status);\nend Push;',
                  'procedure Finish (Owner : in out Thinkthen.Sessions.Session; Error : Reader_Error) is\nbegin\nThinkthen.Native_Sessions.Finish (Owner, Native.Unwrap (Error));\nend Finish;']
    call_body.insert(1, 'with Ada.Unchecked_Conversion;\nwith Interfaces.C.Strings;\nwith Thinkthen.Session_C;\nwith Thinkthen.Requests.Native;\nwith System;')
    call_spec.insert(3, 'type Plan_Estimate is private;\ntype Token_Band is record Lower, Upper : Interfaces.Unsigned_64; end record;\ntype Optional_Text (Present : Boolean := False) is record\ncase Present is when True => Value : Ada.Strings.Unbounded.Unbounded_String; when False => null; end case;\nend record;')
    for member, field in plan_definitions['plan']['properties'].items():
        raw = 'Thinkthen.Values.Field_Value (Item.Data, "' + member + '")'
        method = member.title()
        kind = field.get('type')
        if '$ref' in field:
            target = 'Token_Band'
            value = '(Lower => Interfaces.Unsigned_64\'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (' + raw + ', "lower"))), Upper => Interfaces.Unsigned_64\'Value (Thinkthen.Values.As_Number (Thinkthen.Values.Field_Value (' + raw + ', "upper"))))'
        elif isinstance(kind, list):
            target = 'Optional_Text'
            value = '(if Thinkthen.Values.Kind (' + raw + ') = Thinkthen.Values.Null_Value then (Present => False) else (Present => True, Value => Ada.Strings.Unbounded.To_Unbounded_String (Thinkthen.Values.As_String (' + raw + '))))'
        else:
            target = {'integer':'Interfaces.Unsigned_64', 'boolean':'Boolean', 'string':'String'}[kind]
            value = {'integer':"Interfaces.Unsigned_64'Value (Thinkthen.Values.As_Number (" + raw + '))', 'boolean':'Thinkthen.Values.As_Boolean (' + raw + ')', 'string':'Thinkthen.Values.As_String (' + raw + ')'}[kind]
        signature = 'function ' + method + ' (Item : Plan_Estimate) return ' + target
        call_spec.append(signature + ';')
        call_body.append(signature + ' is (' + value + ');')
    call_body.insert(next(i for i, line in enumerate(call_body) if line.startswith('package body')) + 1, 'use type Thinkthen.Values.Value_Kind;')
    function_names = ', '.join('when Function_Kind_' + name.title() + ' => "' + name + '"' for name in enum_values(definitions['RequestFunction']))
    call_body.append('function Plan (Client : Engine; Function_Name : Function_Kind; Question : Thinkthen.Inputs.Question; Input : Thinkthen.Inputs.Input; Options : Call_Options := No_Options) return Plan_Estimate is\nbegin\nreturn (Data => Thinkthen.Native_Sessions.Plan (Client, (T_question => Native.Unwrap (Question), T_input => Native.Unwrap (Input), T_options => Native.Unwrap (Options), others => <>), (case Function_Name is ' + function_names + ')));\nend Plan;')
    call_body.append('procedure Configure (Client : in out Engine; Settings : Engine_Settings; Error : out Failure) is\nuse Thinkthen.Session_C;\nuse Interfaces.C;\nStorage : Thinkthen.Requests.Native.Arena;\ntype Settings_Access is access constant thinkthen_request_engine_settings_v1;\nfunction Pointer is new Ada.Unchecked_Conversion (System.Address, Settings_Access);\nTree : constant Settings_Access := Pointer (Thinkthen.Requests.Native.Convert (Native.Unwrap (Settings), Storage));\nReplacement : constant Engine_Access := Engine_Access (thinkthen_engine_new_typed (Tree));\nCode : int;\nbegin\nif Replacement = null then\nCode := thinkthen_error_code (null);\nif Code not in 1 .. 6 then raise Program_Error with "unknown native error kind"; end if;\nError := (Kind => Error_Kind\'Val (Code), Retryable => thinkthen_error_retryable (null) /= 0,\nText => Ada.Strings.Unbounded.To_Unbounded_String (Interfaces.C.Strings.Value (thinkthen_error_message (null))));\nelse\nthinkthen_engine_free (Engine_Pointer (Client.Handle));\nClient.Handle := Engine_Address (Replacement);\nError := (others => <>);\nend if;\nend Configure;')
    call_spec += ['private', 'type Plan_Estimate is record Data : Thinkthen.Values.Value; end record;']
    call_spec.append('end Thinkthen.Inputs.Calls;'); call_body.append('end Thinkthen.Inputs.Calls;')
    return {'thinkthen-inputs.ads':spec, 'thinkthen-inputs.adb':body, 'thinkthen-inputs-native.ads':bridge_spec,'thinkthen-inputs-native.adb':bridge_body, 'thinkthen-inputs-calls.ads':'\n'.join(call_spec)+'\n', 'thinkthen-inputs-calls.adb':'\n'.join(call_body)+'\n'}

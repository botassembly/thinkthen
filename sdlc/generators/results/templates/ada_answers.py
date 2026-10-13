"""Own native result values and derive answer routes from the common contract."""
from templates.c import Target, ident, normalized, container_key, nullable, variant_member
from templates.ada import ada_string


def readers(definitions):
    target = Target(definitions)
    for key, source in definitions.items():
        target.definition(key, source)
    declarations, bodies, emitted = [], [], set()

    def expression(source, path, value):
        source = normalized(source)
        if '$ref' in source:
            return emit(source['$ref'].removeprefix('#/$defs/'), definitions[source['$ref'].removeprefix('#/$defs/')]) + '(' + value + ')'
        if source == {} or source.get('type') == 'null':
            return 'Read_Json (' + value + ')'
        if any(key in source for key in ('enum','variants','primitive_variants','properties')):
            return emit(path, source) + '(' + value + ')'
        kind = source.get('type')
        if kind == 'string' or isinstance(source.get('const'), str):
            return 'String_Item (Text (' + value + '))'
        if kind == 'boolean':
            return 'Boolean_Item (' + value + ' /= 0)'
        if kind in ('integer','number'):
            typ = 'double' if kind == 'number' else ('Interfaces.Unsigned_64' if source.get('minimum', -1) >= 0 or source.get('format','').startswith('uint') else 'Interfaces.Integer_64')
            return "Number_Item (Ada.Strings.Fixed.Trim (" + typ + "'Image (" + value + '), Ada.Strings.Both))'
        return emit(container_key(source,path), source, True) + '(' + value + ')'

    def emit(key, source, container=False):
        typ = ident(key)
        function = 'Read_' + typ
        if typ in emitted:
            return function
        emitted.add(typ)
        source = normalized(source)
        signature = f'function {function} (P : ' + (typ if container else 'access constant ' + typ) + ') return Value'
        declarations.append(signature + ';')
        locals, lines = [], []
        if not container:
            lines.append('if P = null then raise Constraint_Error with "missing native value"; end if;')
        if source.get('variants'):
            lines.append('case P.kind is')
            for index,(child,_) in enumerate(source['variants'],1):
                call = emit(child,definitions[child])
                member = variant_member(key,child)
                if member in ('array','null'): member = 'c_' + member
                lines.append(f'when {index} => return {call} (P.data.{member});')
            lines += ['when others => raise Constraint_Error with "invalid native alternative";', 'end case;']
        elif source.get('primitive_variants'):
            lines.append('case P.kind is')
            for index,kind in enumerate(source['primitive_variants'],1):
                item = source.get('primitive_schemas',{}).get(kind,{'type':kind})
                value = 'Null_Item' if kind == 'null' else expression(item,key+'_'+kind,'P.data.'+('c_array' if kind == 'array' else kind))
                lines.append(f'when {index} => return {value};')
            lines += ['when others => raise Constraint_Error with "invalid native alternative";', 'end case;']
        elif 'enum' in source:
            lines.append('case P.kind is')
            lines += [f'when {index} => return String_Item ({ada_string(value)});' for index,value in enumerate(source['enum'],1)]
            lines += ['when others => raise Constraint_Error with "invalid native enum";', 'end case;']
        elif 'properties' in source:
            locals += [f"Fields : Field_List (1 .. {len(source['properties'])} + (if P = null then 0 else Index (P.extensions.len)));", 'Count : Natural := 0;']
            for member,item in source['properties'].items():
                native = 'P.' + ('c_' + member if member in ('function','type','end','record','at') else member + '_u' if member in ('true','false') else member)
                presence = member not in source.get('required',[]) or nullable(item, definitions)
                value = expression(item,key+'_field_'+member,native+('.value' if presence else ''))
                if presence:
                    lines += [f'case {native}.presence is', 'when 0 => null;', f'when 1 => Count := Count + 1; Fields (Count) := Named ({ada_string(member)}, Null_Item);', f'when 2 => Count := Count + 1; Fields (Count) := Named ({ada_string(member)}, {value});', 'when others => raise Constraint_Error with "invalid native presence";', 'end case;']
                else:
                    lines += [f'Count := Count + 1; Fields (Count) := Named ({ada_string(member)}, {value});']
            lines += ['declare', 'Extension_Count : constant Natural := Index (P.extensions.len);', 'type Extension_Array is array (1 .. Extension_Count) of thinkthen_complete_extension_v1 with Convention => C;', 'type Extension_Access is access constant Extension_Array;', 'function Pointer is new Ada.Unchecked_Conversion (System.Address, Extension_Access);', 'Extensions : Extension_Access;', 'begin', "if Extension_Count > 0 then Extensions := Pointer (P.extensions.data.all'Address); end if;", 'for I in 1 .. Extension_Count loop', 'Count := Count + 1; Fields (Count) := Named (Text (Extensions (I).name), Read_Json (Extensions (I).value));', 'end loop;', 'end;', 'return Object_Item (Fields (1 .. Count));']
        elif container:
            mapping = source.get('type') == 'object'
            item = source.get('additionalProperties', {}) if mapping else source.get('items',{})
            item_type,_ = target.shape(item,key+('_entry_value' if mapping else '_item'),'item')
            ada_type = 'access constant ' + item_type.removeprefix('*const ') if item_type.startswith('*const ') else {'u32':'Interfaces.Unsigned_32','u64':'Interfaces.Unsigned_64','i64':'Interfaces.Integer_64','f64':'double'}.get(item_type,item_type)
            if mapping:
                ada_type = ident(key+'_entry')
            locals += ['Count : constant Natural := Index (P.len);', f'type Items_Type is array (1 .. Count) of {ada_type} with Convention => C;', 'type Items_Access is access constant Items_Type;', 'function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);', 'Items : Items_Access;', ('Fields : Field_List (1 .. Count);' if mapping else 'Items_Copy : Value_List (1 .. Count);')]
            # Pointer-to-pointer slices use System.Address in the GNAT header.
            data = 'P.data' if item_type.startswith('*const ') and not mapping else 'P.data.all\'Address'
            lines += [f'if Count > 0 then Items := Pointer ({data}); end if;', 'for I in 1 .. Count loop']
            value = expression(item,key+('_entry_value' if mapping else '_item'),'Items (I).value' if mapping else 'Items (I)')
            lines.append('Fields (I) := Named (Text (Items (I).name), '+value+');' if mapping else 'Items_Copy (I) := '+value+';')
            lines += ['end loop;', 'return Object_Item (Fields);' if mapping else 'return Array_Item (Items_Copy);']
        else:
            lines.append('return ' + expression(source,key+'_value','P.value') + ';')
        bodies.append(signature + ' is\n' + '\n'.join(locals) + '\nbegin\n' + '\n'.join(lines) + '\nend ' + function + ';')
        return function

    emit('completesessionPacket', definitions['completesessionPacket'])
    return '\n'.join(declarations) + '\n' + JSON_READER + '\n' + '\n'.join(bodies)


JSON_READER = '''function Read_Json (P : access constant thinkthen_complete_json_v1) return Value;
function Read_Json (P : access constant thinkthen_complete_json_v1) return Value is
begin
   if P = null then raise Constraint_Error with "missing native value"; end if;
   case P.kind is
      when K_THINKTHEN_COMPLETE_JSON_NULL_V1 => return Null_Item;
      when K_THINKTHEN_COMPLETE_JSON_BOOLEAN_V1 => return Boolean_Item (P.data.boolean /= 0);
      when K_THINKTHEN_COMPLETE_JSON_NUMBER_V1 => return Number_Item (Text (P.data.number));
      when K_THINKTHEN_COMPLETE_JSON_STRING_V1 => return String_Item (Text (P.data.string));
      when K_THINKTHEN_COMPLETE_JSON_ARRAY_V1 =>
         declare
            Count : constant Natural := Index (P.data.c_array.len);
            type Item_Pointer is access constant thinkthen_complete_json_v1;
            type Items_Type is array (1 .. Count) of Item_Pointer with Convention => C;
            type Items_Access is access constant Items_Type;
            function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
            Items : constant Items_Access := Pointer (P.data.c_array.data);
            Copy : Value_List (1 .. Count);
         begin
            for I in 1 .. Count loop Copy (I) := Read_Json (Items (I)); end loop;
            return Array_Item (Copy);
         end;
      when K_THINKTHEN_COMPLETE_JSON_OBJECT_V1 =>
         declare
            Count : constant Natural := Index (P.data.object.len);
            type Items_Type is array (1 .. Count) of thinkthen_complete_json_entry_v1 with Convention => C;
            type Items_Access is access constant Items_Type;
            function Pointer is new Ada.Unchecked_Conversion (System.Address, Items_Access);
            Items : Items_Access;
            Copy : Field_List (1 .. Count);
         begin
            if Count > 0 then Items := Pointer (P.data.object.data.all'Address); end if;
            for I in 1 .. Count loop Copy (I) := Named (Text (Items (I).name), Read_Json (Items (I).value)); end loop;
            return Object_Item (Copy);
         end;
      when others => raise Constraint_Error with "invalid native value kind";
   end case;
end Read_Json;
'''


def outputs(definitions):
    from templates.host_answers import aggregate_answers, scalar_answers, judgment_route
    scalar = scalar_answers(definitions)
    route, guards = judgment_route(definitions)
    declarations, implementations = [], []
    for member,source in definitions['completeFacts']['properties'].items():
        while isinstance(source,dict) and '$ref' in source:
            source = definitions[source['$ref'].removeprefix('#/$defs/')]
        allows_null = nullable(source,definitions)
        source = normalized(source)
        kind = source.get('type')
        if kind not in ('string','boolean','integer','number'):
            continue
        method = '_'.join(part.title() for part in member.split('_'))
        if member in ('requests_sent','records'):
            continue
        target,convert = {'string':('String','As_String'), 'boolean':('Boolean','As_Boolean'), 'integer':('Interfaces.Unsigned_64',"Interfaces.Unsigned_64'Value"), 'number':('Long_Float',"Long_Float'Value")}[kind]
        optional = member not in definitions['completeFacts'].get('required',[]) or allows_null
        field_type = {'string':'Text_Field','boolean':'Boolean_Field','integer':'Count_Field','number':'Number_Field'}[kind]
        declarations.append(f'function {method} (Item : Facts) return ' + (field_type if optional else target) + ';')
        raw = f'Field_Value (Item.Data, "{member}")'
        value = convert + ' (' + ('As_Number (' + raw + ')' if kind in ('integer','number') else raw) + ')'
        if optional:
            if kind == 'string':value = 'To_Unbounded_String (' + value + ')'
            implementations.append(f'function {method} (Item : Facts) return {field_type} is\nbegin\nif not Has_Field (Item.Data, "{member}") then return (State => Missing_Field); end if;\nif Kind ({raw}) = Null_Value then return (State => Null_Field); end if;\nreturn (State => Present_Field, Value => {value});\nend {method};')
        else:
            implementations.append(f'function {method} (Item : Facts) return {target} is ({value});')
    functions = aggregate_answers(definitions)
    declarations += ['type Function_Kind is (' + ', '.join(function.title() + '_Function' for function,_,_ in functions) + ');', 'function Retained_Values (Call : Completion; Function_Name : Function_Kind) return Thinkthen.Values.Value_List;']
    retained = ['function Retained_Values (Call : Completion; Function_Name : Function_Kind) return Value_List is', 'Item : Value;', 'begin', 'case Function_Name is']
    kinds = {function:kind for function,kind,_ in scalar}
    for function,_,_ in functions:
        retained.append(f'when {function.title()}_Function => Item := Answer (Call, "{function}", "{kinds.get(function, "")}", True);')
    retained += ['end case;', 'declare Copy : Value_List (1 .. Length (Item)); begin', "for I in Copy'Range loop Copy (I) := Element (Item, I); end loop;", 'return Copy;', 'end;', 'end Retained_Values;']
    implementations.append('\n'.join(retained))
    for function,kind,schema in scalar:
        target = {'decide':'Decision','choose':'Choice','score':'Long_Float','tag':'Text_List'}[function]
        declarations.append(f'function {function.title()}_Answer (Call : Completion) return {target};')
        body = [f'function {function.title()}_Answer (Call : Completion) return {target} is', f'Item : constant Value := Answer (Call, "{function}", "{kind}");', 'begin']
        if function == 'decide':
            body += ['if Kind (Item) = Null_Value then return (Known => False); end if;', 'return (Known => True, Value => As_Boolean (Item));']
        elif function == 'choose':
            body += ['if Kind (Item) = Null_Value then return (Known => False); end if;', 'return (Known => True, Value => To_Unbounded_String (As_String (Item)));']
        elif function == 'score':
            body += ["return Long_Float'Value (As_Number (Item));"]
        else:
            body += ['return Texts (Item);']
        implementations.append('\n'.join(body + ['end '+function.title()+'_Answer;']))
    for function,_,payload in aggregate_answers(definitions):
        if function in {function for function,_,_ in scalar}:
            continue
        target = {'filter':'Boolean_List','rank':'Rank_List','find':'Find_Result','annotate':'Annotation_List','recognize':'Recognition_List','relate':'Relation'}[function]
        declarations.append(f'function {function.title()}_Answer (Call : Completion) return {target};')
        body = [f'function {function.title()}_Answer (Call : Completion) return {target} is', f'Item : constant Value := Answer (Call, "{function}", "");', 'begin']
        if function in ('filter','rank'):
            body += ['declare', f'Items : {target} (1 .. Length (Item));', 'begin', 'for I in Items\'Range loop', 'Items (I) := '+('As_Boolean' if function == 'filter' else "Interfaces.Unsigned_64'Value")+' ('+('Field_Value (Element (Item, I), "value")' if function == 'filter' else 'As_Number (Field_Value (Element (Item, I), "value"))')+');', 'end loop;', 'return Items;', 'end;']
        elif payload.get('type') == 'array':
            singular = target.removesuffix('_List')
            body += ['declare', f'Items : {target} (1 .. Length (Item));', 'begin', 'for I in Items\'Range loop Items (I) := (Data => Element (Item, I)); end loop;', 'return Items;', 'end;']
        else:
            body += ['return (Data => Item);']
        implementations.append('\n'.join(body+['end '+function.title()+'_Answer;']))
    for function,_,_ in functions:
        target = {'decide':'Decision','choose':'Choice','score':'Long_Float','tag':'Text_List','filter':'Boolean_List','rank':'Rank_List','find':'Find_Result','annotate':'Annotation_List','recognize':'Recognition_List','relate':'Relation'}[function]
        signature = f'function {function.title()}_Answer (Owner : in out Thinkthen.Sessions.Session; Call : in out Completion) return {target}'
        declarations.append(signature + ';')
        implementations.append(signature + ' is\nbegin\nCall := Collect (Owner);\nreturn ' + function.title() + '_Answer (Call);\nend ' + function.title() + '_Answer;')
    guard_lines = []
    def path(item,fields):
        for field in fields: item = f'Field_Value ({item}, "{field}")'
        return item
    for fields,value in guards:
        guard_lines.append(f'if As_String ({path("Packet",fields)}) /= "{value}" then return False; end if;')
    scalar_route = path('Packet',route)
    collections = [function for function,_,payload in aggregate_answers(definitions) if payload.get('type') == 'array' and function not in {function for function,_,_ in scalar}]
    body = BODY.replace('-- READERS', readers(definitions)).replace('-- GUARDS', '\n'.join(guard_lines)).replace('-- SCALAR_ROUTE',scalar_route).replace('-- COLLECTIONS',' or else '.join(f'Function_Name = "{function}"' for function in collections)).replace('-- ANSWERS','\n'.join(implementations))
    return {'thinkthen-answers.ads': SPEC.replace('-- ANSWERS','\n'.join(declarations)), 'thinkthen-answers.adb':body}


SPEC = '''-- Generated owned answers from the canonical native result graph.
with Ada.Strings.Unbounded;
with Interfaces;
with Thinkthen.Values;
with Thinkthen.Sessions;
package Thinkthen.Answers is
   type Completion is private;
   type Completion_Status is (Succeeded, Failed, Malformed);
   Answer_Cardinality_Error : exception;
   Session_Failure : exception;
   -- Drain through terminal. The returned completion owns its successful prefix.
   function Collect (Owner : in out Thinkthen.Sessions.Session) return Completion;
   function Status (Call : Completion) return Completion_Status;
   function Packet_Count (Call : Completion) return Natural;
   type Field_State is (Missing_Field, Null_Field, Present_Field);
   type Count_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Interfaces.Unsigned_64; when others => null; end case;
   end record;
   type Number_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Long_Float; when others => null; end case;
   end record;
   type Boolean_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Boolean; when others => null; end case;
   end record;
   type Text_Field (State : Field_State := Missing_Field) is record
      case State is when Present_Field => Value : Ada.Strings.Unbounded.Unbounded_String; when others => null; end case;
   end record;
   type Facts is private;
   type Failure is private;
   function Has_Facts (Call : Completion) return Boolean;
   function Call_Facts (Call : Completion) return Facts;
   function Call_Failure (Call : Completion) return Failure;
   function Kind (Error : Failure) return Thinkthen.Sessions.Error_Status;
   function Message (Error : Failure) return String;
   function Retryable (Error : Failure) return Boolean;
   function Has_Facts (Error : Failure) return Boolean;
   function Failure_Facts (Error : Failure) return Facts;
   function Requests_Sent (Item : Facts) return Interfaces.Unsigned_64;
   function Records (Item : Facts) return Interfaces.Unsigned_64;
   function Details (Item : Facts) return Thinkthen.Values.Value;
   function Details (Error : Failure) return Thinkthen.Values.Value;
   type Decision (Known : Boolean := False) is record
      case Known is
         when True => Value : Boolean;
         when False => null;
      end case;
   end record;
   type Choice (Known : Boolean := False) is record
      case Known is
         when True => Value : Ada.Strings.Unbounded.Unbounded_String;
         when False => null;
      end case;
   end record;
   type Text_List is array (Positive range <>) of Ada.Strings.Unbounded.Unbounded_String;
   type Boolean_List is array (Positive range <>) of Boolean;
   type Rank_List is array (Positive range <>) of Interfaces.Unsigned_64;
   type Find_Result is private;
   type Annotation is private;
   type Recognition is private;
   type Relation is private;
   type Annotation_List is array (Positive range <>) of Annotation;
   type Recognition_List is array (Positive range <>) of Recognition;
   function Found_Value (Item : Find_Result) return Thinkthen.Values.Value;
   function Has_Match (Item : Find_Result) return Boolean;
   function Match_Index (Item : Find_Result) return Interfaces.Unsigned_64;
   function Fields (Item : Annotation) return Thinkthen.Values.Value;
   function Entities (Item : Recognition) return Thinkthen.Values.Value;
   function Edges (Item : Relation) return Thinkthen.Values.Value;
   function Source_Name (Edge : Thinkthen.Values.Value) return String;
   function Target_Name (Edge : Thinkthen.Values.Value) return String;
   function Relation_Name (Edge : Thinkthen.Values.Value) return String;
   function Details (Item : Find_Result) return Thinkthen.Values.Value;
   function Details (Item : Annotation) return Thinkthen.Values.Value;
   function Details (Item : Recognition) return Thinkthen.Values.Value;
   function Details (Item : Relation) return Thinkthen.Values.Value;
   -- ANSWERS
private
   type Completion is record
      Packets : Thinkthen.Values.Value;
      Terminal : Thinkthen.Values.Value;
      Terminal_Count : Natural := 0;
      Valid : Boolean := True;
   end record;
   type Facts is record Data : Thinkthen.Values.Value; end record;
   type Failure is record Data : Thinkthen.Values.Value; end record;
   type Find_Result is record Data : Thinkthen.Values.Value; end record;
   type Annotation is record Data : Thinkthen.Values.Value; end record;
   type Recognition is record Data : Thinkthen.Values.Value; end record;
   type Relation is record Data : Thinkthen.Values.Value; end record;
end Thinkthen.Answers;
'''

BODY = '''-- Generated owned native answers. Do not edit.
with Ada.Containers.Vectors;
with Ada.Unchecked_Conversion;
with Ada.Strings.Fixed;
with System;
with Interfaces.C;
with Thinkthen_Session_C;
package body Thinkthen.Answers is
   use Ada.Strings.Unbounded;
   use Thinkthen.Values;
   use Thinkthen.Sessions;
   use Thinkthen_Session_C;
   use Interfaces.C;
   use type Interfaces.Unsigned_32;
   package Packet_Lists is new Ada.Containers.Vectors (Positive, Value);
   -- READERS
   function Collect (Owner : in out Session) return Completion is
      Owned : Packet_Lists.Vector;
      Packet : Thinkthen.Sessions.Packet;
      Read : Read_Status;
      Result : Completion;
   begin
      loop
         Next_Packet (Owner, Packet, Read);
         exit when Read = Finished;
         if Read /= Thinkthen.Sessions.Result then
            Result.Valid := False;
         else
            declare
               Item : constant Value := Read_thinkthen_complete_session_packet_v1 (View (Packet));
            begin
               if As_String (Field_Value (Item, "kind")) = "terminal" then
                  Result.Terminal_Count := Result.Terminal_Count + 1;
                  Result.Terminal := Item;
               else
                  if Result.Terminal_Count > 0 then Result.Valid := False; end if;
                  Owned.Append (Item);
               end if;
            end;
         end if;
      end loop;
      declare
         Items : Value_List (1 .. Natural (Owned.Length));
      begin
         for I in Items'Range loop Items (I) := Owned (I); end loop;
         Result.Packets := Array_Item (Items);
      end;
      return Result;
   end Collect;
   function Status (Call : Completion) return Completion_Status is
   begin
      if not Call.Valid or else Call.Terminal_Count /= 1 then return Malformed; end if;
      if Has_Field (Call.Terminal, "failure") then
         if Kind (Field_Value (Call.Terminal, "failure")) /= Object_Value then return Malformed; end if;
         return Failed;
      end if;
      return Succeeded;
   end Status;
   function Packet_Count (Call : Completion) return Natural is (Length (Call.Packets));
   function Has_Facts (Call : Completion) return Boolean is
     (Call.Terminal_Count = 1 and then Has_Field (Call.Terminal, "facts") and then
       Kind (Field_Value (Call.Terminal, "facts")) = Object_Value);
   function Call_Facts (Call : Completion) return Facts is ((Data => Field_Value (Call.Terminal, "facts")));
   function Call_Failure (Call : Completion) return Failure is ((Data => Field_Value (Call.Terminal, "failure")));
   function Details (Item : Facts) return Value is (Item.Data);
   function Details (Error : Failure) return Value is (Error.Data);
   function Kind (Error : Failure) return Error_Status is
      Name : constant String := As_String (Field_Value (Field_Value (Error.Data, "error"), "kind"));
   begin
      return Error_Status'Value (Name);
   end Kind;
   function Message (Error : Failure) return String is (As_String (Field_Value (Field_Value (Error.Data, "error"), "message")));
   function Retryable (Error : Failure) return Boolean is (As_Boolean (Field_Value (Field_Value (Error.Data, "error"), "retryable")));
   function Has_Facts (Error : Failure) return Boolean is (Has_Field (Error.Data, "facts") and then Kind (Field_Value (Error.Data, "facts")) = Object_Value);
   function Failure_Facts (Error : Failure) return Facts is ((Data => Field_Value (Error.Data, "facts")));
   function Requests_Sent (Item : Facts) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "requests_sent"))));
   function Records (Item : Facts) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "records"))));
   function Scalar_Packet (Packet : Value; Judgment : String) return Boolean is
   begin
      -- GUARDS
      return As_String (Field_Value (-- SCALAR_ROUTE, "kind")) = Judgment;
   exception
      when Constraint_Error => return False;
   end Scalar_Packet;
   function Answer (Call : Completion; Function_Name, Judgment : String; Retained : Boolean := False) return Value is
      package Answer_Lists is new Ada.Containers.Vectors (Positive, Value);
      Items : Answer_Lists.Vector;
      Collection : constant Boolean := -- COLLECTIONS;
   begin
      if not Retained then
      case Status (Call) is
         when Failed => raise Session_Failure with Message (Call_Failure (Call));
         when Malformed => raise Answer_Cardinality_Error with "expected exactly one valid terminal";
         when Succeeded => null;
      end case;
      end if;
      for I in 1 .. Length (Call.Packets) loop
         declare
            Packet : constant Value := Element (Call.Packets, I);
         begin
            if Has_Field (Packet, "function") and then As_String (Field_Value (Packet, "function")) = Function_Name then
               if Judgment /= "" then
                  if Scalar_Packet (Packet, Judgment) then Items.Append (Field_Value (-- SCALAR_ROUTE, "value")); end if;
               elsif Collection and then As_String (Field_Value (Packet, "kind")) = "row" then
                  Items.Append (Field_Value (Packet, "value"));
               elsif As_String (Field_Value (Packet, "kind")) = "aggregate" then
                  declare Payload : constant Value := Field_Value (Packet, "value"); begin
                     if Collection then
                        for J in 1 .. Length (Payload) loop Items.Append (Element (Payload, J)); end loop;
                     else Items.Append (Payload); end if;
                  end;
               end if;
            end if;
         end;
      end loop;
      if Collection or else Retained then
         declare
            Copy : Value_List (1 .. Natural (Items.Length));
         begin
            for I in Copy'Range loop Copy (I) := Items (I); end loop;
            return Array_Item (Copy);
         end;
      end if;
      if Natural (Items.Length) /= 1 then raise Answer_Cardinality_Error with "expected exactly one matching answer"; end if;
      return Items (1);
   end Answer;
   function Texts (Item : Value) return Text_List is
      Result : Text_List (1 .. Length (Item));
   begin
      for I in Result'Range loop Result (I) := To_Unbounded_String (As_String (Element (Item, I))); end loop;
      return Result;
   end Texts;
   function Found_Value (Item : Find_Result) return Value is (Field_Value (Item.Data, "value"));
   function Has_Match (Item : Find_Result) return Boolean is (Kind (Field_Value (Item.Data, "index")) /= Null_Value);
   function Match_Index (Item : Find_Result) return Interfaces.Unsigned_64 is (Interfaces.Unsigned_64'Value (As_Number (Field_Value (Item.Data, "index"))));
   function Fields (Item : Annotation) return Value is (Field_Value (Item.Data, "value"));
   function Entities (Item : Recognition) return Value is (Field_Value (Field_Value (Item.Data, "value"), "entities"));
   function Edges (Item : Relation) return Value is (Field_Value (Item.Data, "value"));
   function Source_Name (Edge : Value) return String is (As_String (Field_Value (Field_Value (Edge, "source"), "name")));
   function Target_Name (Edge : Value) return String is (As_String (Field_Value (Field_Value (Edge, "target"), "name")));
   function Relation_Name (Edge : Value) return String is (As_String (Field_Value (Edge, "relation")));
   function Details (Item : Find_Result) return Value is (Item.Data);
   function Details (Item : Annotation) return Value is (Item.Data);
   function Details (Item : Recognition) return Value is (Item.Data);
   function Details (Item : Relation) return Value is (Item.Data);
   -- ANSWERS
end Thinkthen.Answers;
'''

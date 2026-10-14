"""Generate Ada call records with concrete probabilities and owned receipts."""
from templates.host_answers import call_results


def result_type(function, spec, single):
    return function.title() + ('_Result' if single or spec['call_shape'] != 'input' else '_List_Result')


def outputs(definitions):
    specs = call_results(definitions)
    declarations = ['type Receipt is private;',
                    'type Receipt_List is array (Positive range <>) of Receipt;',
                    'function Record_Value (Item : Receipt) return Thinkthen.Values.Value;',
                    'type Number_Field_List is array (Positive range <>) of Number_Field;',
                    'package Probability_Lists is new Ada.Containers.Vectors (Positive, Number_Field);',
                    'package Nested_Probability_Lists is new Ada.Containers.Vectors (Positive, Probability_Lists.Vector, Probability_Lists."=");']
    bodies = ['function Record_Value (Item : Receipt) return Value is (Item.Data);', HELPERS]
    for function, spec in specs.items():
        array = spec['receipt_payload'].get('type') == 'array'
        mode = spec['probability_mode']
        for single in ([True, False] if spec['call_shape'] == 'input' else [False]):
            name = result_type(function, spec, single)
            scalar_value = single or spec['call_shape'] == 'aggregate'
            list_odds = mode != 'null' and not scalar_value
            item_odds = mode == 'items' and function != 'rank'
            p_type = ('Nested_Probability_Lists.Vector' if item_odds and list_odds else
                      'Probability_Lists.Vector' if item_odds else
                      'Number_Field_List (1 .. Value_Count)' if list_odds else 'Number_Field')
            v_type = 'Thinkthen.Values.Value' if scalar_value else 'Thinkthen.Values.Value_List (1 .. Value_Count)'
            discriminants = 'Record_Count : Natural' if scalar_value else 'Value_Count, Record_Count : Natural'
            declarations += [f'type {name} ({discriminants}) is record', f'Value : {v_type};', f'Probability : {p_type};', 'Results : Receipt_List (1 .. Record_Count);', 'Facts : Thinkthen.Answers.Facts;', 'end record;']
            signature = f'function {function.title()} (Call : Completion) return {name}'
            declarations.append(signature + ';')
            lines = [signature + ' is', f'Rows : constant Value_List := Receipts (Call, "{function}", {str(array)});', f'Result : {name} (' + ('Rows\'Length' if scalar_value else "Rows'Length, Rows'Length") + ');', 'begin']
            if scalar_value:
                lines.append('if Rows\'Length /= 1 then raise Answer_Cardinality_Error with "expected one result"; end if;')
            lines += ['Result.Facts := Call_Facts (Call);', "for I in Rows'Range loop", 'Result.Results (I) := (Data => Rows (I));']
            projection = f'Field_Value (Rows (I), "{spec["value_projection"]}")'
            lines.append(('Result.Value' if scalar_value else 'Result.Value (I)') + ' := ' + projection + ';')
            if item_odds:
                expr = 'Item_Probabilities (Rows (I), ' + str(function == 'recognize') + ')'
                lines.append('Result.Probability := ' + expr + ';' if scalar_value else 'Result.Probability.Append (' + expr + ');')
            elif mode != 'null':
                expr = number_expression(spec, function)
                lines.append(('Result.Probability' if scalar_value else 'Result.Probability (I)') + ' := ' + expr + ';')
            else:
                lines.append('Result.Probability := (State => Null_Field);')
            lines += ['end loop;', 'return Result;', 'end ' + function.title() + ';']
            bodies.append('\n'.join(lines))
    return '\n'.join(declarations), '\n'.join(bodies), 'type Receipt is record Data : Thinkthen.Values.Value; end record;'


def path(item, fields):
    for field in fields:
        item = f'Field_Value ({item}, "{field}")'
    return item


def number_expression(spec, function):
    if spec['probability_mode'] == 'items':
        return 'Number (Field_Value (Field_Value (Rows (I), "answer"), "probability"))'
    value = path('Rows (I)', spec['probability_path'])
    if spec['probability_mode'] == 'picked':
        pick = path('Rows (I)', spec['pick_path'])
        value = f'Element ({value}, Natural\'Value (As_Number ({pick})) + 1)'
    value = 'Number (' + value + ')'
    if 'selection_path' in spec:
        selected = path('Rows (I)', spec['selection_path'])
        return f'(if Kind ({selected}) /= Null_Value then {value} else (State => Null_Field))'
    if 'resolution' in spec:
        resolution = spec['resolution']
        conditions = [f'As_String ({path("Packet", fields)}) = "{literal}"' for fields, literal in resolution['guards']]
        conditions += [f'As_String (Field_Value (Packet, "function")) = "{function}"', f'{path("Packet", resolution["index_path"])} = Row_Index', f'Kind ({path("Packet", resolution["path"])}) /= Null_Value']
        # The native normalized observation owns resolution, including authored nulls.
        resolver = 'Resolved_' + function.title()
        RESOLVERS[resolver] = '\n'.join([f'function {resolver} (Call : Completion; Row : Value; Ordinal : Positive) return Boolean is', 'Row_Index : constant Value := (if Has_Field (Row, "index") then Field_Value (Row, "index") else Number_Item (Natural\'Image (Ordinal - 1)));', 'begin', 'for J in 1 .. Length (Call.Packets) loop', 'declare Packet : constant Value := Element (Call.Packets, J); begin', 'if ' + ' and then '.join(conditions) + ' then return True; end if;', 'exception when Constraint_Error => null; end;', 'end loop;', 'return False;', 'end ' + resolver + ';'])
        return f'(if {resolver} (Call, Rows (I), I) then {value} else (State => Null_Field))'
    return value


RESOLVERS = {}
HELPERS = '''function Receipts (Call : Completion; Function_Name : String; Is_Array : Boolean) return Value_List is
   Owned : Packet_Lists.Vector;
begin
   case Status (Call) is
      when Failed => raise Session_Failure with Message (Call_Failure (Call));
      when Malformed => raise Answer_Cardinality_Error with "expected one valid terminal";
      when Succeeded => null;
   end case;
   for I in 1 .. Length (Call.Packets) loop
      declare Packet : constant Value := Element (Call.Packets, I); begin
         if Has_Field (Packet, "function") and then As_String (Field_Value (Packet, "function")) = Function_Name then
            declare
               Packet_Kind : constant String := As_String (Field_Value (Packet, "kind"));
               Payload : constant Value := Field_Value (Packet, "value");
            begin
               if Packet_Kind = "aggregate" and then Is_Array then
                  for J in 1 .. Length (Payload) loop Owned.Append (Element (Payload, J)); end loop;
               elsif Packet_Kind = "row" or else Packet_Kind = "aggregate" then
                  Owned.Append (Payload);
               end if;
            end;
         end if;
      end;
   end loop;
   declare Result : Value_List (1 .. Natural (Owned.Length)); begin
      for I in Result'Range loop Result (I) := Owned (I); end loop;
      return Result;
   end;
end Receipts;
function Number (Item : Value) return Number_Field is
begin
   if Kind (Item) = Null_Value then return (State => Null_Field); end if;
   return (State => Present_Field, Value => Long_Float'Value (As_Number (Item)));
end Number;
function Item_Probabilities (Row : Value; Recognition : Boolean) return Probability_Lists.Vector is
   Result : Probability_Lists.Vector;
   Payload : constant Value := Field_Value (Row, "value");
begin
   if not Recognition then
      for I in 1 .. Length (Payload) loop
         Result.Append (Number (Field_Value (Element (Payload, I), "probability")));
      end loop;
   elsif Has_Field (Payload, "proposals") then
      declare Proposals : constant Value := Field_Value (Payload, "proposals"); begin
         for I in 1 .. Length (Proposals) loop
            Result.Append (Number (Field_Value (Element (Proposals, I), "probability")));
         end loop;
      end;
   else
      declare
         Entities : constant Value := Field_Value (Payload, "entities");
         Proposals : constant Value := Field_Value (Field_Value (Row, "answer"), "proposals");
      begin
         for I in 1 .. Length (Entities) loop
            declare
               Entity : constant Value := Element (Entities, I);
               Matched : Boolean := False;
            begin
               for J in 1 .. Length (Proposals) loop
                  declare
                     Proposal : constant Value := Element (Proposals, J);
                     Selected : constant Value := Field_Value (Proposal, "selected");
                  begin
                     if As_Boolean (Field_Value (Proposal, "kept")) and then Kind (Selected) /= Null_Value
                       and then Field_Value (Selected, "start") = Field_Value (Entity, "start")
                       and then Field_Value (Selected, "end") = Field_Value (Entity, "end")
                       and then Field_Value (Proposal, "kind") = Field_Value (Entity, "kind") then
                        if Matched then raise Answer_Cardinality_Error with "duplicate selected span"; end if;
                        Result.Append (Number (Field_Value (Proposal, "span_probability")));
                        Matched := True;
                     end if;
                  end;
               end loop;
               if not Matched then raise Answer_Cardinality_Error with "missing selected span"; end if;
            end;
         end loop;
      end;
   end if;
   return Result;
end Item_Probabilities;
'''

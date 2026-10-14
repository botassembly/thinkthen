"""Generate owned C++ call projections from the shared result descriptors."""
import json
from host_answers import call_results


def leaf(schema, definitions, typ):
    if isinstance(schema, dict) and '$ref' in schema:
        target = definitions[schema['$ref'].split('/')[-1]]
        if 'properties' not in target and 'variants' not in target:
            return leaf(target, definitions, typ)
    if isinstance(schema, dict) and isinstance(schema.get('type'), list) and 'null' in schema['type']:
        return 'std::optional<' + typ({**schema, 'type': next(k for k in schema['type'] if k != 'null')}) + '>'
    return typ(schema)


def render(definitions, typ, name):
    lines = ['namespace tt::results {',
             'template<class V,class P,class R> struct CallResult {',
             'V value; P probability; std::vector<R> results; Facts facts;', '};',
             'namespace detail {',
             'inline std::vector<std::optional<double>> recognition_probability(const Data& row) {',
             'std::vector<std::optional<double>> odds; const auto& value=row.at("value");',
             'if(value.contains("proposals")) { for(const auto& proposal:value.at("proposals")) odds.emplace_back(proposal.at("probability").get<double>()); return odds; }',
             'for(const auto& entity:value.at("entities")) { std::optional<double> probability;',
             'for(const auto& proposal:row.at("answer").at("proposals")) {',
             'if(!proposal.at("kept").get<bool>() || !proposal.contains("kind") || !proposal.contains("selected")) continue;',
             'const auto& bounds=proposal.at("selected");',
             'if(bounds.at("start")==entity.at("start") && bounds.at("end")==entity.at("end") && proposal.at("kind")==entity.at("kind")) {',
             'if(probability) throw std::invalid_argument("multiple kept recognition proposals"); probability=proposal.at("span_probability").get<double>(); }',
             '}', 'odds.push_back(probability);', '}', 'return odds;', '}',
             'inline void successful_result(const std::vector<SessionPacket>& packets) {',
             'size_t terminals=0;',
             'for(const auto& packet:packets) if(literal(document(packet),"kind","terminal")) { ++terminals;',
             'if(document(packet).contains("failure")) throw ::tt::Failure("terminal contains a failure",packets); }',
             'if(terminals!=1 || packets.empty() || !literal(document(packets.back()),"kind","terminal") || !document(packets.back()).contains("facts")) throw ::tt::Failure("expected one final successful terminal with facts",packets);', '}', '}',]
    for function, descriptor in call_results(definitions).items():
        schema = descriptor['aggregate_value']
        shape = descriptor['call_shape']
        item = schema['items'] if isinstance(schema, dict) and schema.get('type') == 'array' and shape != 'aggregate' else schema
        value_type = leaf(item, definitions, typ)
        if function == 'recognize':
            value_type = 'std::variant<std::vector<Entity>,std::vector<BoundaryProposal>>'
        aggregate_type = typ(descriptor['receipt_payload'])
        receipt_schema = descriptor['receipt_payload'].get('items', descriptor['receipt_payload'])
        receipt_type = typ(receipt_schema)
        mode = descriptor['probability_mode']
        probability_type = 'std::vector<std::optional<double>>' if mode == 'items' else 'std::optional<double>'
        if shape == 'collection':
            result_value = 'std::vector<' + value_type + '>'
            result_probability = 'std::vector<std::optional<double>>'
        elif shape == 'input':
            result_value = 'std::conditional_t<Single,' + value_type + ',std::vector<' + value_type + '>>'
            result_probability = 'std::conditional_t<Single,' + probability_type + ',std::vector<' + probability_type + '>>' if mode != 'null' else 'std::optional<double>'
        else:
            result_value, result_probability = value_type, probability_type
        lines += (["template<bool Single>"] if shape == 'input' else []) + [f'using {function.title()}Result = CallResult<{result_value},{result_probability},{receipt_type}>;',
                  ('namespace detail {\n' + ('template<bool Single>\n' if shape == 'input' else '') + f'inline {function.title()}Result' + ('<Single>' if shape == 'input' else '') + f' project_{function}(const std::vector<SessionPacket>& packets) {{'),
                  'detail::successful_result(packets);', f'std::optional<{aggregate_type}> aggregate;',
                  f'for(const auto& packet:packets) if(auto carrier=packet.as_{name(descriptor["carrier"])}()) {{',
                  'if(aggregate) throw ::tt::Failure("multiple matching aggregates",packets); aggregate.emplace(*carrier->value().value); }',
                  'if(!aggregate) throw ::tt::Failure("no matching aggregate",packets);',
                  f'std::vector<{receipt_type}> records;']
        if descriptor['receipt_payload'].get('type') == 'array':
            lines += ['records=std::move(*aggregate);']
        else:
            lines += ['records.push_back(std::move(*aggregate));']
        lines += [f'std::vector<{value_type}> values;', f'std::vector<{probability_type}> probabilities;',
                  'for(size_t i=0;i<records.size();++i) { const auto& row=detail::document(records[i]);']
        member = descriptor['value_projection']
        if function == 'recognize':
            expression = 'row.at("value").contains("entities") ? ' + value_type + '(detail::decode<std::vector<Entity>>(row.at("value").at("entities"))) : ' + value_type + '(detail::decode<std::vector<BoundaryProposal>>(row.at("value").at("proposals")))'
        else:
            expression = f'detail::decode<{value_type}>(row.at({json.dumps(member)}))'
        lines += ['values.push_back(' + expression + ');']
        if mode == 'null':
            lines += ['probabilities.emplace_back();']
        elif mode == 'items' and function == 'recognize':
            lines += ['probabilities.push_back(detail::recognition_probability(row));']
        elif mode == 'items' and shape == 'aggregate':
            lines += ['std::vector<std::optional<double>> odds; for(const auto& edge:row.at("value")) odds.emplace_back(edge.at("probability").get<double>()); probabilities.push_back(std::move(odds));']
        else:
            lines += ['bool resolved=true;']
            resolution = descriptor.get('resolution')
            if resolution:
                def access(parts): return 'observation' + ''.join('.at('+json.dumps(p)+')' for p in parts)
                guard = ' && '.join('detail::literal('+access(p[:-1])+','+json.dumps(p[-1])+','+json.dumps(v)+')' for p,v in resolution['guards'])
                lines += ['size_t matched=0; resolved=false;', 'for(const auto& packet:packets) { const auto& observation=detail::document(packet);',
                          f'if({guard} && {access(resolution["index_path"])}==(row.contains("index") ? row.at("index") : Data(uint64_t(i)))) {{',
                          f'++matched; resolved=!{access(resolution["path"])}.is_null();', '}}',
                          'if(matched!=1) throw ::tt::Failure("expected one matching judgment per input",packets);']
            if descriptor.get('selection_path'):
                path = 'row' + ''.join('.at('+json.dumps(p)+')' for p in descriptor['selection_path'])
                lines += ['resolved=!' + path + '.is_null();']
            path = descriptor.get('probability_path')
            if mode == 'picked':
                odds = 'row' + ''.join('.at('+json.dumps(p)+')' for p in path)
                pick = 'row' + ''.join('.at('+json.dumps(p)+')' for p in descriptor['pick_path'])
                expression = f'{odds}.at({pick}.get<std::string>()).get<double>()'
            else:
                path = path or ('answer','probability')
                expression = 'row' + ''.join('.at('+json.dumps(p)+')' for p in path) + '.get<double>()'
            lines += [f'std::optional<double> odds; if(resolved) odds={expression};']
            if mode == 'items':
                lines += ['probabilities.push_back({odds});']
            else:
                lines += ['probabilities.push_back(odds);']
        lines += ['}', 'auto facts=detail::decode<Facts>(detail::document(packets.back()).at("facts"));']
        if shape == 'input':
            lines += ['if constexpr(Single) { if(values.size()!=1) throw ::tt::Failure("expected one scalar result",packets);',
                      'return {std::move(values.front()),' + ('std::optional<double>{}' if mode=='null' else 'std::move(probabilities.front())') + ',std::move(records),std::move(facts)}; } else {',
                      'return {std::move(values),' + ('std::optional<double>{}' if mode=='null' else 'std::move(probabilities)') + ',std::move(records),std::move(facts)}; }']
        elif shape == 'collection':
            lines += []
            odds = 'probabilities' if mode != 'items' else 'odds'
            if mode == 'items':
                lines += ['std::vector<std::optional<double>> odds; for(auto& item:probabilities) odds.push_back(item.front());']
            lines += [f'return {{std::move(values),std::move({odds}),std::move(records),std::move(facts)}};']
        else:
            lines += ['if(values.size()!=1) throw ::tt::Failure("expected one whole-set result",packets);',
                      'return {std::move(values.front()),std::move(probabilities.front()),std::move(records),std::move(facts)};']
        lines += ['}', '}']
    return '\n'.join(lines+['}'])+'\n'

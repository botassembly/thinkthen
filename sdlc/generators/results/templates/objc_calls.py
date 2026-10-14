"""Generate Foundation call projections from canonical result descriptors."""
from host_answers import call_results, scalar_input_selectors


def call_helpers(definitions, inputs):
    from objc import conversion, literal, shape
    def value_conversion(raw, expression):
        source = shape(raw)
        if "$ref" in source:
            source = shape(definitions[source["$ref"].split("/")[-1]])
        if "primitive_variants" in source:
            return "id", expression
        if source.get("type") == "object" and isinstance(source.get("additionalProperties"), dict):
            _, decode = value_conversion(source["additionalProperties"], "item")
            return "NSDictionary *", f"TTMap({expression}, ^id(id item) {{ return {decode}; }})"
        return conversion(raw, definitions, expression)
    calls = call_results(definitions)
    def at(path, root='fields'):
        return root + ''.join('[' + literal(part) + ']' for part in path)
    header = [
        '@interface TTCallValue : NSObject',
        '@property(nonatomic, strong, readonly, nullable) id scalar;',
        '@property(nonatomic, copy, readonly, nullable) NSArray *items;',
        '- (instancetype)init NS_UNAVAILABLE;', '+ (instancetype)new NS_UNAVAILABLE;', '@end',
        '@interface TTCallProbability : NSObject',
        '@property(nonatomic, strong, readonly, nullable) NSNumber *scalar;',
        '@property(nonatomic, copy, readonly, nullable) NSArray<TTCallProbability *> *items;',
        '- (instancetype)init NS_UNAVAILABLE;', '+ (instancetype)new NS_UNAVAILABLE;', '@end']
    implementation = [r'''@interface TTCallValue ()
- (instancetype)initWithScalar:(id)scalar items:(NSArray *)items;
@end
@implementation TTCallValue
- (instancetype)initWithScalar:(id)scalar items:(NSArray *)items {
    if ((self = [super init])) { _scalar = scalar; _items = [items copy]; } return self;
}
@end
@interface TTCallProbability ()
- (instancetype)initWithScalar:(NSNumber *)scalar items:(NSArray<TTCallProbability *> *)items;
@end
@implementation TTCallProbability
- (instancetype)initWithScalar:(NSNumber *)scalar items:(NSArray<TTCallProbability *> *)items {
    if ((self = [super init])) { _scalar = scalar; _items = [items copy]; } return self;
}
@end
static TTCallProbability *TTProbability(id value) {
    if (!value || value == NSNull.null) return nil;
    if ([value isKindOfClass:NSArray.class]) return [[TTCallProbability alloc] initWithScalar:nil items:TTArray(value, ^id(id item) { return TTProbability(item) ?: [[TTCallProbability alloc] initWithScalar:nil items:nil]; })];
    return [[TTCallProbability alloc] initWithScalar:TTNumber(value) items:nil];
}
static id TTPickedProbability(NSDictionary *row) {
    NSDictionary *answer = TTObject(row[@"answer"]);
    if ([answer[@"kind"] isEqual:@"yes_no"]) return answer[@"probability"] ?: NSNull.null;
    if ([answer[@"kind"] isEqual:@"choice"]) return TTObject(answer[@"probabilities"])[answer[@"pick"]] ?: NSNull.null;
    return NSNull.null;
}
static NSArray *TTRecognitionProbabilities(NSDictionary *row) {
    NSDictionary *value = TTObject(row[@"value"]);
    if (value[@"proposals"]) return TTArray(value[@"proposals"], ^id(id item) { return TTObject(item)[@"probability"]; });
    NSArray *proposals = TTObject(row[@"answer"])[@"proposals"];
    return TTArray(value[@"entities"], ^id(id item) {
        NSDictionary *entity = TTObject(item);
        for (NSDictionary *proposal in proposals) {
            NSDictionary *bounds = proposal[@"selected"] ?: proposal;
            if ([proposal[@"kept"] boolValue] && [bounds[@"start"] isEqual:entity[@"start"]] &&
                [bounds[@"end"] isEqual:entity[@"end"]] && [proposal[@"kind"] isEqual:entity[@"kind"]])
                return proposal[@"span_probability"];
        }
        TTInvalidResult();
    });
}
BOOL TTCallSingleInput(TTInput *input) {''']
    kinds = [inputs[key]['properties']['kind']['const'] for key in scalar_input_selectors(inputs)]
    implementation += ['return ' + ' || '.join('[input.requestFields[@"kind"] isEqual:' + literal(kind) + ']' for kind in kinds) + ';', '}']
    implementation += ['static NSDictionary *TTCallProjection(TTCall *call) {',
                       'NSMutableArray *rows = [NSMutableArray array];',
                       'NSMutableDictionary<NSNumber *, NSNumber *> *resolved = [NSMutableDictionary dictionary];',
                       'id aggregate = nil;',
                       'for (TTSessionPacket *packet in call.packets) { NSDictionary *fields = packet.rawFields;',
                       'if (![fields[@"function"] isEqual:call.function]) continue;',
                       'if ([fields[@"kind"] isEqual:@"row"]) [rows addObject:fields[@"value"]];',
                       'if ([fields[@"kind"] isEqual:@"aggregate"]) aggregate = fields[@"value"];']
    for function, spec in calls.items():
        resolution = spec.get('resolution')
        if not resolution: continue
        guards = ' && '.join('[' + at(path) + ' isEqual:' + literal(value) + ']' for path, value in resolution['guards'])
        implementation += ['if ([call.function isEqual:' + literal(function) + '] && ' + guards + ') resolved[' + at(resolution['index_path']) + '] = @(' + at(resolution['path']) + ' != NSNull.null);']
    implementation += ['}', 'id value = nil, probability = nil;', 'NSArray<TTResultNode *> *results = nil;']
    for function, spec in calls.items():
        payload = spec['receipt_payload']
        is_array = payload.get('type') == 'array'
        record_schema = payload['items'] if is_array else payload
        _, record_decode = conversion(record_schema, definitions, 'item')
        projection = spec['value_projection']
        value_schema = spec['aggregate_value']['items'] if is_array else spec['aggregate_value']
        _, value_decode = value_conversion(value_schema, 'raw')
        implementation += ['if ([call.function isEqual:' + literal(function) + ']) {']
        if is_array:
            implementation += ['if (aggregate) rows = [aggregate mutableCopy];']
            if projection == 'input':
                implementation += ['rows = [[rows filteredArrayUsingPredicate:[NSPredicate predicateWithBlock:^BOOL(id item, NSDictionary *bindings) { (void)bindings; return [TTObject(item)[@"value"] boolValue]; }]] mutableCopy];']
        else:
            implementation += ['if (aggregate) [rows addObject:aggregate];']
        implementation += ['results = TTArray(rows, ^id(id item) { return ' + record_decode + '; });',
                           'if (!rows.count && call.terminal.failure.state == TTPresenceValue) return @{@"value":[[TTCallValue alloc] initWithScalar:nil items:nil], @"probability":NSNull.null, @"results":results};',
                           'NSMutableArray *values = [NSMutableArray array]' + (', *odds = [NSMutableArray array];' if spec['probability_mode'] != 'null' else ';'),
                           'for (NSDictionary *row in rows) { id raw = row[' + literal(projection) + '];',
                           'if (!raw) TTInvalidResult();',
                           *(['BOOL settled = raw != NSNull.null;'] if spec['probability_mode'] in ('yes', 'picked') else [])]
        if projection == 'input' and spec['probability_mode'] == 'yes':
            implementation += ['settled = [row[@"value"] boolValue];']
        if 'resolution' in spec:
            implementation += ['settled = [resolved[row[@"index"] ?: @0] boolValue];']
        if 'selection_path' in spec:
            implementation += ['settled = ' + at(spec['selection_path'], 'row') + ' != NSNull.null;']
        implementation += ['[values addObject:raw == NSNull.null ? NSNull.null : ' + value_decode + '];']
        mode = spec['probability_mode']
        if mode in ('yes', 'picked'):
            prob = at(spec['probability_path'], 'row')
            if mode == 'picked': prob += '[' + at(spec['pick_path'], 'row') + ']'
            implementation += ['[odds addObject:settled ? (' + prob + ' ?: NSNull.null) : NSNull.null];']
        elif mode == 'items':
            if function == 'rank': implementation += ['[odds addObject:TTPickedProbability(row)];']
            elif function == 'recognize': implementation += ['[odds addObject:TTRecognitionProbabilities(row)];']
            else: implementation += ['[odds addObject:TTArray(raw, ^id(id item) { return TTObject(item)[@"probability"]; })];']
        implementation += ['}']
        scalar = spec['call_shape'] == 'input'
        if scalar:
            implementation += ['if (call.singleInput) { if (values.count != 1) TTInvalidResult();',
                               'id raw = values[0]; BOOL settled = raw != NSNull.null;']
            if 'resolution' in spec: implementation += ['settled = [resolved[rows[0][@"index"] ?: @0] boolValue];']
            implementation += ['value = [[TTCallValue alloc] initWithScalar:settled ? raw : nil items:nil];']
            if mode != 'null': implementation += ['probability = TTProbability(odds[0]);']
            implementation += ['} else { value = [[TTCallValue alloc] initWithScalar:nil items:values];']
            if mode != 'null': implementation += ['probability = TTProbability(odds);']
            implementation += ['}']
        elif is_array:
            implementation += ['value = [[TTCallValue alloc] initWithScalar:nil items:values];']
            if mode != 'null': implementation += ['probability = TTProbability(odds);']
        else:
            implementation += ['if (values.count != 1) TTInvalidResult();']
            if function == 'find':
                implementation += ['value = [[TTCallValue alloc] initWithScalar:TTObject(rows[0])[@"index"] == NSNull.null ? nil : values[0] items:nil];',
                                   'probability = TTProbability(odds[0]);']
            else:
                implementation += ['value = [[TTCallValue alloc] initWithScalar:nil items:values[0]];', 'probability = TTProbability(odds[0]);']
        implementation += ['}']
    implementation += ['if (!value || !results) TTInvalidResult();',
                       'return @{@"value":value, @"probability":probability ?: NSNull.null, @"results":results};', '}',
                       '@implementation TTCall (Values)',
                       '- (TTCallValue *)value { return TTCallProjection(self)[@"value"]; }',
                       '- (TTCallProbability *)probability { id result = TTCallProjection(self)[@"probability"]; return result == NSNull.null ? nil : result; }',
                       '- (NSArray<TTResultNode *> *)results { return TTCallProjection(self)[@"results"]; }', '@end']
    return header, implementation

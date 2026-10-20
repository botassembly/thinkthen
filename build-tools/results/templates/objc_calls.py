"""Generate mechanical readers for canonical Call JSON."""


def call_helpers(definitions, inputs):
    from objc import conversion, literal
    properties = definitions['completeCallSuccess']['properties']
    header = ['@interface TTCall : TTResultNode']
    implementation = ['@implementation TTCall',
        '+ (instancetype)read:(id)value { return [[self alloc] initWithFields:TTObject(value)]; }']
    # Canonical value is the detailed receipt; answer fields are already settled by Rust.
    fields = [('facts', properties['facts'], 'self.rawFields', 'facts'),
              ('results', properties['value'], 'self.rawFields', 'value')]
    for field, schema, root, key in fields:
        target, decode = conversion(schema, definitions)
        header += [f'@property(nonatomic, readonly) TTPresence<{target}> *{field};']
        implementation += [f'- (TTPresence<{target}> *){field} {{ return [self presence:{literal(key)} required:YES convert:^id(id value) {{ return {decode}; }}]; }}']
    header += ['@property(nonatomic, readonly) TTPresence<TTSettledAnswer *> *answer;']
    implementation += ['- (TTPresence<TTSettledAnswer *> *)answer { return [self presence:@"answer" required:NO convert:^id(id value) { return [TTSettledAnswer read:value]; }]; }']
    for field in ('value', 'probability', 'pick', 'index', 'resolution'):
        header += [f'@property(nonatomic, readonly) TTPresence<id> *{field};']
        implementation += [f'- (TTPresence<id> *){field} {{ TTResultNode *answer = [[TTResultNode alloc] initWithFields:self.rawFields[@"answer"] ?: @{{}}]; return [answer presence:{literal(field)} required:NO convert:^id(id value) {{ return value; }}]; }}']
    return header + ['@end'], implementation + ['@end']

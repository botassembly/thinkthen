part of 'engine.dart';

extension _Questions on Engine {
  CChoicesView _choices(List<Choice> values, _Scope scope) {
    final a =
        scope.add(calloc<CChoiceView>(values.isEmpty ? 1 : values.length));
    for (var i = 0; i < values.length; ++i) {
      final v = values[i];
      a[i].name = scope.string(v.name);
      a[i].description = scope.optional(v.description);
      if (v.weight != null) {
        a[i].weight.present = 1;
        a[i].weight.value = v.weight!;
      }
    }
    final out = scope.add(calloc<CChoicesView>()).ref;
    out.data = a;
    out.len = values.length;
    return out;
  }

  CStringsView _strings(List<String> values, _Scope scope) {
    final a =
        scope.add(calloc<CStringView>(values.isEmpty ? 1 : values.length));
    for (var i = 0; i < values.length; ++i) {
      a[i] = scope.string(values[i]);
    }
    final out = scope.add(calloc<CStringsView>()).ref;
    out.data = a;
    out.len = values.length;
    return out;
  }

  COptionalStringView _optionalString(String? value, _Scope scope) {
    final out = scope.add(calloc<COptionalStringView>()).ref;
    if (value != null) {
      out.present = 1;
      out.value = scope.string(value);
    }
    return out;
  }

  CRuleView _rule(Rule? value, _Scope scope) {
    final out = scope.add(calloc<CRuleView>()).ref;
    if (value != null) {
      out.kind = value.kind;
      out.low = value.low;
      out.high = value.high;
    }
    return out;
  }

  CInputDeclarationView _declaration(Declaration? v, _Scope scope) {
    final out = scope.add(calloc<CInputDeclarationView>()).ref;
    if (v == null) return out;
    out.kind = v.kind;
    out.required = _strings(v.required, scope);
    final a = scope.add(calloc<CInputPropertyView>(
        v.properties.isEmpty ? 1 : v.properties.length));
    for (var i = 0; i < v.properties.length; ++i) {
      a[i].name = scope.string(v.properties[i].name);
      a[i].kind = v.properties[i].kind.index + 1;
    }
    out.properties.data = a;
    out.properties.len = v.properties.length;
    return out;
  }

  Pointer<CQuestionAuthorView> _author(Author? v, _Scope scope) {
    final out = scope.add(calloc<CQuestionAuthorView>());
    if (v == null) return out;
    out.ref.name = _optionalString(v.name, scope);
    if (v.wordingVersion != null) {
      final n = v.wordingVersion!;
      if (n.isNegative || n.bitLength > 64)
        throw ArgumentError('unsigned wording version');
      out.ref.wording_version.present = 1;
      out.ref.wording_version.value = n.toSigned(64).toInt();
    }
    out.ref.item_schema = _declaration(v.itemSchema, scope);
    out.ref.context_schema = _declaration(v.contextSchema, scope);
    return out;
  }

  Pointer<Void> _constructed(QuestionSpec v, _Scope scope) {
    final s = scope.add(calloc<CQuestionSpecView>());
    s.ref.kind = v.kind.index + 1;
    if (v.text != null) s.ref.text = scope.content(v.text!);
    s.ref.yes = scope.optional(v.yes);
    s.ref.no = scope.optional(v.no);
    s.ref.choices = _choices(v.choices, scope);
    s.ref.threshold = _rule(v.threshold, scope);
    s.ref.relation_threshold = _rule(v.relationThreshold, scope);
    s.ref.model = _optionalString(v.model, scope);
    s.ref.profile = _optionalString(v.profile, scope);
    if (v.batch != null) {
      s.ref.batch.present = 1;
      s.ref.batch.value = v.batch!;
    }
    s.ref.batch_max = v.batchMax ? 1 : 0;
    s.ref.none = v.none ? 1 : 0;
    s.ref.on = _strings(v.on, scope);
    final members = scope
        .add(calloc<CMemberSpecView>(v.members.isEmpty ? 1 : v.members.length));
    final children = <Pointer<Void>>[];
    try {
      for (var i = 0; i < v.members.length; ++i) {
        members[i].name = scope.string(v.members[i].name);
        final child = _question(v.members[i].question, scope);
        children.add(child);
        members[i].question = child;
      }
      s.ref.members.data = members;
      s.ref.members.len = v.members.length;
      s.ref.kinds = _choices(v.kinds, scope);
      final rels = scope.add(
          calloc<CRelationView>(v.relations.isEmpty ? 1 : v.relations.length));
      for (var i = 0; i < v.relations.length; ++i) {
        final r = v.relations[i];
        rels[i].name = scope.string(r.name);
        rels[i].source = scope.string(r.source);
        rels[i].target = scope.string(r.target);
        rels[i].reads = _optionalString(r.reads, scope);
        rels[i].either = r.either ? 1 : 0;
        rels[i].single = r.single ? 1 : 0;
      }
      s.ref.relations.data = rels;
      s.ref.relations.len = v.relations.length;
      s.ref.name_pointer = _optionalString(v.namePointer, scope);
      s.ref.kind_pointer = _optionalString(v.kindPointer, scope);
      final out = scope.add(calloc<Pointer<Void>>());
      _check(_api.thinkthen_question_new_authored(
          _engine, s, _author(v.author, scope), out));
      return out.value;
    } finally {
      for (final child in children) {
        _api.thinkthen_question_free(child);
      }
    }
  }
}

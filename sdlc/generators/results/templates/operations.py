"""Derive native exports from operation contracts, with no embedded bodies."""
from itertools import combinations
import re


def abi_type(value):
    aliases = {'EngineHandle': 'crate::EngineHandle',
               'SessionHandle': 'crate::session::SessionHandle',
               'SessionResultHandle': 'crate::session::SessionResultHandle',
               'c_char': 'std::ffi::c_char',
               'NamedRequestV1': 'crate::ffi::session::NamedRequestV1',
               'thinkthen_complete_utf8_v1': 'crate::session::views::thinkthen_complete_utf8_v1'}
    value = re.sub(r'(?<!:)\b(?:EngineHandle|SessionHandle|SessionResultHandle|c_char|NamedRequestV1|thinkthen_complete_utf8_v1)\b', lambda match: aliases[match[0]], value)
    return value.replace('carriers::', 'super::carriers::')



def render(operations, statuses=(), projections=()):
    lines = ['//! Generated native operation transport. Do not edit.',
             '#![allow(unsafe_code, reason = "foreign slots follow the operation ownership contract")]',
             '#![deny(unsafe_op_in_unsafe_fn)]', 'use crate::failures;']
    if any(operation.diagnostic in ('session_call', 'session_native_call') for operation in operations):
        lines.append('use thinkthen::ErrorKind;')
    for name, value, documentation in statuses:
        lines.extend(['/// ' + documentation, 'pub const ' + name + ': u32 = ' + str(value) + ';'])
    for name, native, variants in projections:
        lines.extend(['pub(crate) fn ' + name + '(value: ' + native + ') -> u32 {', '    match value {'])
        lines.extend('        ' + native + '::' + variant + ' => ' + constant + ',' for variant, constant in variants)
        lines.extend(['    }', '}'])
    for operation in operations:
        names = {argument.name for argument in operation.arguments}
        for argument in operation.arguments:
            if argument.count and argument.count not in names:
                raise ValueError('counted operation input has no length argument')
        if not set(operation.outputs) <= names:
            raise ValueError('operation output has no ABI argument')
        lines.extend(operation.documentation.rstrip().splitlines())
        lines.extend(['#[unsafe(no_mangle)]', 'pub ' + ('unsafe ' if operation.foreign_unsafe else '') + 'extern "C" fn ' + operation.name + '('])
        lines.extend('    ' + argument.name + ': ' + abi_type(argument.abi) + ',' for argument in operation.arguments)
        lines.append(')' + ('' if operation.result == '()' else ' -> ' + abi_type(operation.result)) + ' {')
        if operation.diagnostic == 'engine_guard':
            if operation.fallback not in ('NO_MESSAGE.as_ptr()', 'std::ptr::null()', '0', 'DEFECT'):
                raise ValueError('unknown engine diagnostic fallback')
            lines.extend(['        // SAFETY: the caller retains its nullable engine owner.',
                          '        let held = unsafe { super::held(engine) };',
                          '        super::guard(held, ' + ('super::' + operation.fallback if operation.fallback in ('NO_MESSAGE.as_ptr()', 'DEFECT') else operation.fallback) + ', || failures::' + operation.native + '(held))', '}', ''])
            continue
        if operation.diagnostic in ('session_guard', 'constructor_guard'):
            if operation.fallback not in ('()', '0', 'std::ptr::null()', 'c"defect: a session operation failed".as_ptr()'):
                raise ValueError('unknown ownership or diagnostic fallback')
            guard = 'failures::session_guard(' if operation.diagnostic == 'session_guard' else 'super::guard(None, '
            if not operation.arguments and operation.native.startswith('failures::'):
                lines.extend(['    ' + guard + operation.fallback + ', ' + operation.native + ')', '}', ''])
                continue
            lines.append('    ' + guard + operation.fallback + ', || {')
            if operation.native in ('release_box', 'release_cstring'):
                if len(operation.arguments) != 1 or operation.arguments[0].ownership != 'consume-nullable':
                    raise ValueError('release requires one transferred nullable owner')
                owner = operation.arguments[0].name
                allocator = 'Box' if operation.native == 'release_box' else 'std::ffi::CString'
                lines.extend(['        if !' + owner + '.is_null() {',
                    '            // SAFETY: the owner came from this allocator and all borrowers have finished.',
                    '            drop(unsafe { ' + allocator + '::from_raw(' + owner + ') });', '        }'])
            else:
                native = operation.native if operation.native.startswith('failures::') else 'super::' + operation.native
                call = native + '(' + ', '.join(operation.inputs) + ')'
                if operation.native_unsafe:
                    lines.append('        // SAFETY: the caller retains the nullable owner throughout the helper.')
                    call = 'unsafe { ' + call + ' }'
                lines.append('        ' + call)
            lines.extend(['    })', '}', ''])
            continue
        lines.append('    failures::' + operation.diagnostic + '(|| {')
        usage = 'ErrorKind::Usage' + ('.into()' if operation.diagnostic == 'session_native_call' else '')
        for slot in operation.outputs:
            lines.extend(['        if ' + slot + '.is_null() {',
                          '            return Err(' + usage + ');', '        }'])
        for left, right in combinations(operation.outputs, 2):
            lines.extend(['        if ' + left + '.cast::<()>() == ' + right + '.cast::<()>() {',
                          '            return Err(' + usage + ');', '        }'])
        for argument in operation.arguments:
            if argument.ownership == 'borrow':
                lines.extend(['        // SAFETY: the caller retains this live owner throughout the operation.',
                              '        let ' + argument.name + ' = unsafe { ' + argument.name + '.as_ref() }.ok_or(ErrorKind::Usage)?;'])
        allowed_literals = {'@None', '@Some(interest)', '@false', '@true', '@thinkthen::Engine::usage_persistence', '@thinkthen::Engine::finish_usage_status'}
        if any(value.startswith('@') and value not in allowed_literals for value in operation.inputs):
            raise ValueError('operation inputs must name arguments or registered native selectors')
        inputs = ', '.join(value.removeprefix('@') for value in operation.inputs)
        result = 'super::' + operation.native + '(' + inputs + ')'
        if operation.native_unsafe:
            result = 'unsafe { ' + result + ' }'
            lines.append('        // SAFETY: the caller retains counted input trees until the native helper returns.')
        if not operation.outputs:
            lines.extend(['        ' + result, '    })', '}', ''])
            continue
        binding = '(' + ', '.join('value_' + slot for slot in operation.outputs) + ')' if len(operation.outputs) > 1 else 'value_' + operation.outputs[0]
        lines.append('        let ' + binding + ' = ' + result + '?;')
        lines.extend(['        // SAFETY: every slot was validated before the native operation; success publishes once.', '        unsafe {'])
        lines.extend('            *' + slot + ' = value_' + slot + ';' for slot in operation.outputs)
        lines.extend(['        }', '        Ok(())', '    })', '}', ''])
    return '\n'.join(lines)

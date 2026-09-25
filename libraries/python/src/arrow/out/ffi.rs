//! Answers out through the C data interface. `ArrayNode` and `SchemaNode`
//! trees become C structs here. Every node of an emitted batch carries one
//! share of the batch's memory, so a consumer that moves a child out and
//! releases the parent still owns the child, as the interface's rules for
//! moving children require.

use std::collections::VecDeque;
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::ptr::{self, NonNull};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyCapsule;

use super::ffi::{
    ARRAY, ArrowArray, ArrowArrayStream, ArrowSchema, EMPTY_ARRAY, EMPTY_SCHEMA, Imported, SCHEMA,
    STREAM,
};
use super::write::{ArrayNode, SchemaNode};

/// One emitted batch's memory: every struct boxed so its address holds, the
/// pointer tables, the owned buffers, and the caller's frames it aliases.
#[derive(Debug, Default)]
struct Keep {
    #[expect(
        clippy::vec_box,
        reason = "each struct's address must hold while the Vec grows"
    )]
    arrays: Vec<Box<ArrowArray>>,
    tables: Vec<Vec<*const c_void>>,
    children: Vec<Vec<*mut ArrowArray>>,
    buffers: Vec<Vec<u8>>,
    holds: Vec<Arc<Imported>>,
    /// Which of `arrays` are aliases, whose children stay the producer's.
    aliases: Vec<*mut ArrowArray>,
}

// SAFETY: a `Keep` owns every struct, table, and buffer its pointers name,
// and nothing mutates it once it is shared. The C data interface lets a
// consumer release a batch on any thread, and the caller's frames it holds
// are `Imported`, which any thread may hold and release.
unsafe impl Send for Keep {}
// SAFETY: as above.
unsafe impl Sync for Keep {}

/// Lay one node out in `keep` and give its struct's address.
fn lay_out(node: ArrayNode, keep: &mut Keep) -> *mut ArrowArray {
    let mut array = match node {
        ArrayNode::Alias(alias) => {
            keep.holds.push(alias.hold);
            let mut boxed = Box::new(ArrowArray {
                release: None,
                private_data: ptr::null_mut(),
                ..alias.array
            });
            let at: *mut ArrowArray = &raw mut *boxed;
            keep.aliases.push(at);
            keep.arrays.push(boxed);
            return at;
        }
        ArrayNode::Owned(owned) => {
            let table: Vec<*const c_void> = owned
                .buffers
                .iter()
                .map(|buffer| {
                    buffer
                        .as_ref()
                        .map_or(ptr::null(), |bytes| bytes.as_ptr().cast())
                })
                .collect();
            keep.buffers.extend(owned.buffers.into_iter().flatten());
            let children: Vec<*mut ArrowArray> = owned
                .children
                .into_iter()
                .map(|child| lay_out(child, keep))
                .collect();
            let array = Box::new(ArrowArray {
                length: owned.length,
                null_count: owned.null_count,
                n_buffers: i64::try_from(table.len()).unwrap_or(0),
                n_children: i64::try_from(children.len()).unwrap_or(0),
                buffers: table.as_ptr().cast_mut(),
                children: if children.is_empty() {
                    ptr::null_mut()
                } else {
                    children.as_ptr().cast_mut()
                },
                ..EMPTY_ARRAY
            });
            keep.tables.push(table);
            keep.children.push(children);
            array
        }
    };
    let at: *mut ArrowArray = &raw mut *array;
    keep.arrays.push(array);
    at
}

/// A C struct this module hands out: its release and its private data.
trait Node: Sized {
    fn slots(
        &mut self,
    ) -> (
        &mut Option<unsafe extern "C" fn(*mut Self)>,
        &mut *mut c_void,
    );
}

impl Node for ArrowArray {
    fn slots(
        &mut self,
    ) -> (
        &mut Option<unsafe extern "C" fn(*mut Self)>,
        &mut *mut c_void,
    ) {
        (&mut self.release, &mut self.private_data)
    }
}

impl Node for ArrowSchema {
    fn slots(
        &mut self,
    ) -> (
        &mut Option<unsafe extern "C" fn(*mut Self)>,
        &mut *mut c_void,
    ) {
        (&mut self.release, &mut self.private_data)
    }
}

/// Give every node one share of `keep` and the release `pick` names, then
/// move the root into `out`. The root's box keeps no share of its own.
///
/// # Safety
/// Every node and the root name boxes `keep` owns, and `out` is writable.
unsafe fn hand_out<K, N: Node>(
    keep: K,
    nodes: Vec<*mut N>,
    (root, out): (*mut N, *mut N),
    pick: impl Fn(*mut N) -> unsafe extern "C" fn(*mut N),
) {
    let share = Arc::new(keep);
    // SAFETY: every pointer names a box the share keeps, and `out` is the
    // caller's writable struct.
    unsafe {
        for node in nodes {
            let (release, data) = (*node).slots();
            *data = Box::into_raw(Box::new(Arc::clone(&share))).cast();
            *release = Some(pick(node));
        }
        *out = ptr::read(root);
        let (release, data) = (*root).slots();
        *release = None;
        *data = ptr::null_mut();
    }
}

/// Drop the share of `K` a node carries and mark it released, so a second
/// call frees nothing.
///
/// # Safety
/// `node` is one `hand_out` gave a share of `K`.
unsafe fn drop_share<K, N: Node>(node: *mut N) {
    // SAFETY: `node` is one this module handed out, and its share is taken
    // once.
    unsafe {
        let (release, data) = (*node).slots();
        *release = None;
        let share = std::mem::replace(data, ptr::null_mut()).cast::<Arc<K>>();
        if !share.is_null() {
            drop(Box::from_raw(share));
        }
    }
}

/// Emit one batch into `out`: an alias keeps its producer's children.
///
/// # Safety
/// `out` is a writable struct.
unsafe fn emit(node: ArrayNode, out: *mut ArrowArray) {
    let mut keep = Keep::default();
    let root = lay_out(node, &mut keep);
    let nodes: Vec<*mut ArrowArray> = keep.arrays.iter_mut().map(|one| &raw mut **one).collect();
    let aliases = keep.aliases.clone();
    let pick = |node| {
        if aliases.contains(&node) {
            alias_release
        } else {
            owned_release
        }
    };
    // SAFETY: the nodes and root are `keep`'s boxes; `out` is writable.
    unsafe { hand_out(keep, nodes, (root, out), pick) };
}

/// An owned node's release: release each child the consumer has not moved
/// out (a moved child reads released), then drop the node's share.
unsafe extern "C" fn owned_release(node: *mut ArrowArray) {
    // SAFETY: `node` is one this module emitted. Its children table names
    // `n_children` boxes the share keeps.
    unsafe {
        let count = usize::try_from((*node).n_children).unwrap_or(0);
        for place in 0..count {
            let child = *(*node).children.add(place);
            if let Some(release) = (*child).release {
                release(child);
            }
        }
        drop_share::<Keep, _>(node);
    }
}

/// An alias's release drops its share only. Its children are the producer's,
/// and the caller's frame releases them with its last share.
unsafe extern "C" fn alias_release(node: *mut ArrowArray) {
    // SAFETY: as `drop_share`.
    unsafe { drop_share::<Keep, _>(node) };
}

/// One schema tree handed to a consumer, with the strings it points at.
#[derive(Debug, Default)]
struct SchemaKeep {
    #[expect(
        clippy::vec_box,
        reason = "each struct's address must hold while the Vec grows"
    )]
    nodes: Vec<Box<ArrowSchema>>,
    tables: Vec<Vec<*mut ArrowSchema>>,
    strings: Vec<CString>,
    blobs: Vec<Vec<u8>>,
}

fn lay_out_schema(node: &SchemaNode, keep: &mut SchemaKeep) -> *mut ArrowSchema {
    let children: Vec<*mut ArrowSchema> = node
        .children
        .iter()
        .map(|child| lay_out_schema(child, keep))
        .collect();
    let dictionary = node
        .dictionary
        .as_deref()
        .map_or(ptr::null_mut(), |held| lay_out_schema(held, keep));
    let format = node.format.clone();
    let name = node.name.clone();
    let metadata = node.metadata.clone();
    let mut schema = Box::new(ArrowSchema {
        format: format.as_ptr(),
        name: name.as_ref().map_or(ptr::null(), |text| text.as_ptr()),
        metadata: metadata
            .as_ref()
            .map_or(ptr::null(), |blob| blob.as_ptr().cast()),
        flags: node.flags,
        n_children: i64::try_from(children.len()).unwrap_or(0),
        children: if children.is_empty() {
            ptr::null_mut()
        } else {
            children.as_ptr().cast_mut()
        },
        dictionary,
        release: None,
        private_data: ptr::null_mut(),
    });
    keep.strings.push(format);
    keep.strings.extend(name);
    keep.blobs.extend(metadata);
    keep.tables.push(children);
    let at: *mut ArrowSchema = &raw mut *schema;
    keep.nodes.push(schema);
    at
}

// SAFETY: as for `Keep`: a `SchemaKeep` owns what its pointers name, and
// nothing mutates it once it is shared.
unsafe impl Send for SchemaKeep {}
// SAFETY: as above.
unsafe impl Sync for SchemaKeep {}

/// Write a fresh copy of `node` into `out`, owned by the consumer. Every node
/// carries one share of the tree, so a child the consumer moves out outlives
/// its parent's release.
///
/// # Safety
/// `out` is a writable struct.
unsafe fn hand_schema(node: &SchemaNode, out: *mut ArrowSchema) {
    let mut keep = SchemaKeep::default();
    let root = lay_out_schema(node, &mut keep);
    let nodes: Vec<*mut ArrowSchema> = keep.nodes.iter_mut().map(|one| &raw mut **one).collect();
    // SAFETY: the nodes and root are `keep`'s boxes; `out` is writable.
    unsafe { hand_out(keep, nodes, (root, out), |_| schema_release) };
}

/// A schema node's release: release each child and the dictionary the
/// consumer has not moved out, then drop the node's share.
unsafe extern "C" fn schema_release(node: *mut ArrowSchema) {
    // SAFETY: `node` is one this module handed out. Its children table names
    // `n_children` boxes the share keeps, and its share is taken once.
    unsafe {
        let count = usize::try_from((*node).n_children).unwrap_or(0);
        let dictionary = (*node).dictionary;
        let children = (0..count).map(|place| *(*node).children.add(place));
        for child in children.chain((!dictionary.is_null()).then_some(dictionary)) {
            if let Some(release) = (*child).release {
                release(child);
            }
        }
        drop_share::<SchemaKeep, _>(node);
    }
}

/// The state behind an output stream.
#[derive(Debug)]
struct Out {
    schema: SchemaNode,
    batches: VecDeque<ArrayNode>,
}

unsafe extern "C" fn out_schema(stream: *mut ArrowArrayStream, out: *mut ArrowSchema) -> c_int {
    // SAFETY: a stream this module made, whose private data is its `Out`.
    unsafe { hand_schema(&(*(*stream).private_data.cast::<Out>()).schema, out) };
    0
}

unsafe extern "C" fn out_next(stream: *mut ArrowArrayStream, out: *mut ArrowArray) -> c_int {
    // SAFETY: as `out_schema`. An ended stream writes a released struct.
    unsafe {
        match (*(*stream).private_data.cast::<Out>()).batches.pop_front() {
            Some(batch) => emit(batch, out),
            None => *out = EMPTY_ARRAY,
        }
    }
    0
}

unsafe extern "C" fn out_error(_stream: *mut ArrowArrayStream) -> *const c_char {
    ptr::null()
}

unsafe extern "C" fn out_release(stream: *mut ArrowArrayStream) {
    // SAFETY: a stream this module made, released once.
    unsafe {
        (*stream).release = None;
        let state = (*stream).private_data.cast::<Out>();
        (*stream).private_data = ptr::null_mut();
        if !state.is_null() {
            drop(Box::from_raw(state));
        }
    }
}

/// A capsule's destructor: release the struct if the consumer did not take
/// it, then free the box.
macro_rules! destructor {
    ($name:ident, $kind:ty, $label:expr) => {
        pub(super) unsafe extern "C" fn $name(capsule: *mut pyo3::ffi::PyObject) {
            // SAFETY: a capsule this module made over a boxed struct.
            unsafe {
                let held =
                    pyo3::ffi::PyCapsule_GetPointer(capsule, $label.as_ptr()).cast::<$kind>();
                if held.is_null() {
                    pyo3::ffi::PyErr_Clear();
                    return;
                }
                if let Some(release) = (*held).release {
                    release(held);
                }
                drop(Box::from_raw(held));
            }
        }
    };
}

destructor!(stream_destructor, ArrowArrayStream, STREAM);
destructor!(schema_destructor, ArrowSchema, SCHEMA);
destructor!(array_destructor, ArrowArray, ARRAY);

pub(super) fn capsule(
    py: Python<'_>,
    held: *mut c_void,
    name: &'static CStr,
    destructor: unsafe extern "C" fn(*mut pyo3::ffi::PyObject),
) -> PyResult<Py<PyAny>> {
    let Some(held) = NonNull::new(held) else {
        return Err(crate::defect(py, "an Arrow capsule had no struct"));
    };
    // SAFETY: `held` is a boxed struct of `name`'s kind, which `destructor`
    // releases and frees.
    let made =
        unsafe { PyCapsule::new_with_pointer_and_destructor(py, held, name, Some(destructor)) }?;
    Ok(made.into_any().unbind())
}

/// A stream capsule over whole batches.
pub(super) fn export_stream(
    py: Python<'_>,
    schema: SchemaNode,
    batches: Vec<ArrayNode>,
) -> PyResult<Py<PyAny>> {
    let state = Box::new(Out {
        schema,
        batches: batches.into(),
    });
    let stream = Box::new(ArrowArrayStream {
        get_schema: Some(out_schema),
        get_next: Some(out_next),
        get_last_error: Some(out_error),
        release: Some(out_release),
        private_data: Box::into_raw(state).cast(),
    });
    capsule(py, Box::into_raw(stream).cast(), STREAM, stream_destructor)
}

/// A schema capsule and an array capsule for one column.
pub(super) fn export_array(
    py: Python<'_>,
    schema: &SchemaNode,
    array: ArrayNode,
) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
    let mut out_schema = Box::new(EMPTY_SCHEMA);
    let mut out_array = Box::new(EMPTY_ARRAY);
    // SAFETY: both boxes are writable structs.
    unsafe {
        hand_schema(schema, &raw mut *out_schema);
        emit(array, &raw mut *out_array);
    }
    Ok((
        capsule(
            py,
            Box::into_raw(out_schema).cast(),
            SCHEMA,
            schema_destructor,
        )?,
        capsule(py, Box::into_raw(out_array).cast(), ARRAY, array_destructor)?,
    ))
}

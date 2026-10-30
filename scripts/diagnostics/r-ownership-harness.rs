// Appended inside the exact upstream ownership module for a scratch-only witness.
#[no_mangle]
pub unsafe extern "C" fn ownership_source_identity(expected: SEXP) -> SEXP {
    let matches = LENGTH(expected) as usize == WITNESS_SOURCE_ID.len()
        && WITNESS_SOURCE_ID.bytes().enumerate().all(|(i, byte)| {
            extendr_ffi::INTEGER_ELT(expected, i as R_xlen_t) == i32::from(byte)
        });
    extendr_ffi::Rf_ScalarInteger(i32::from(matches))
}

#[no_mangle]
pub unsafe extern "C" fn ownership_witness(case: SEXP) -> SEXP {
    let result = std::panic::catch_unwind(|| {
        let live_case = extendr_ffi::INTEGER_ELT(case, 0) == 1;
        let count = if live_case {
            INITIAL_PRESERVATION_SIZE + 1
        } else {
            INITIAL_PRESERVATION_SIZE * 2 + 1
        };
        let pool = Rf_protect(Rf_allocVector(SEXPTYPE::VECSXP, count as R_xlen_t));
        // Ordinary R ownership prevents address recycling; no R collection is requested.
        for i in 0..count {
            let value = extendr_ffi::Rf_ScalarInteger(i as i32);
            SET_VECTOR_ELT(pool, i as R_xlen_t, value);
        }
        let mut own = Ownership::new();
        let initial_map_capacity = own.objects.capacity();
        let mut peak_live = 0;
        for i in 0..count {
            let value = VECTOR_ELT(pool, i as R_xlen_t);
            own.protect(value);
            own.protect(value);
            assert_eq!(own.ref_count(value), 2);
            own.unprotect(value);
            assert_eq!(own.ref_count(value), 1);
            if live_case {
                peak_live = i + 1;
            } else {
                peak_live = 1;
                own.unprotect(value);
                assert_eq!(own.ref_count(value), 0);
            }
        }
        own.check_objects();
        let live = own.objects.values().filter(|object| object.refcount != 0).count();
        let passed = if live_case {
            assert_eq!(live, count);
            for i in 0..count {
                let value = VECTOR_ELT(pool, i as R_xlen_t);
                assert_eq!(own.ref_count(value), 1);
                own.unprotect(value);
            }
            own.check_objects();
            assert!(own.objects.values().all(|object| object.refcount == 0));
            own.max_index >= count
        } else {
            assert_eq!(live, 0);
            own.max_index <= INITIAL_PRESERVATION_SIZE + 2 * peak_live
                && own.objects.capacity() <= initial_map_capacity
        };
        println!("{{\"case\":\"{}\",\"objects\":{},\"peak_live\":{},\"live_at_boundary_end\":{},\"slots\":{},\"vector_length\":{},\"map_capacity\":{},\"initial_map_capacity\":{},\"slot_refcounts_consistent\":true,\"passed\":{}}}",
            if live_case { "live" } else { "inactive" }, count, peak_live, live,
            own.max_index, LENGTH(own.preservation.inner()), own.objects.capacity(),
            initial_map_capacity, passed);
        R_ReleaseObject(own.preservation.inner());
        Rf_unprotect(1);
        passed
    });
    extendr_ffi::Rf_ScalarInteger(i32::from(result.unwrap_or(false)))
}

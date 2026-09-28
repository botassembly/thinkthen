DO $thinkthen_grant$
DECLARE signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d
          ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e
          ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('GRANT EXECUTE ON FUNCTION %s TO %I',
                       signature, 'the_app_role');
    END LOOP;
END
$thinkthen_grant$;

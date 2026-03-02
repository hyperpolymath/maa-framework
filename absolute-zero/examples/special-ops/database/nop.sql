-- ============================================================================
-- NOP.SQL - SQL Queries with No Net Effect
-- ============================================================================
-- Demonstrates database operations that consume resources but produce no
-- observable state change. These are "Absolute Zero" operations at the
-- database level - they execute successfully but leave no trace in data.
--
-- CNO Characteristics:
-- - ACID compliant but effectively null operations
-- - Transaction log overhead without state mutation
-- - Lock acquisition and release with no data modification
-- - Query plan execution with zero-row impact
-- - Statistics/index consultation without updates
-- ============================================================================

-- ----------------------------------------------------------------------------
-- POSTGRESQL EXAMPLES
-- ----------------------------------------------------------------------------

-- NOP 1: Self-Canceling UPDATE
-- Acquires row locks, generates undo logs, but changes nothing
-- Query plan includes: Seq Scan → Filter → Update (0 rows)
BEGIN;
UPDATE users SET last_login = last_login WHERE 1 = 0;
-- Transaction log entry: BEGIN + UPDATE (0 rows) + COMMIT
-- Locks: ACCESS EXCLUSIVE on table metadata (momentary)
-- ACID: Atomic (yes), Consistent (yes), Isolated (yes), Durable (no change)
COMMIT;

-- NOP 2: Tautological DELETE
-- Table scan executed, no rows deleted
-- Index consulted for optimization, but not modified
DELETE FROM users WHERE id IS NULL AND id IS NOT NULL;
-- Query plan: Index Scan → Filter (contradiction detected) → Delete (0 rows)
-- Statistics: Table access counter incremented, cardinality unchanged
-- Locks: Row-level locks attempted on 0 rows

-- NOP 3: INSERT...RETURNING with ROLLBACK
-- Generates transaction ID, allocates tuple, then discards
BEGIN;
INSERT INTO audit_log (event, timestamp)
VALUES ('probe', CURRENT_TIMESTAMP)
RETURNING id;
-- MVCC: New tuple version created with transaction visibility
-- Transaction log: Full INSERT record written
ROLLBACK;
-- Result: Tuple marked as aborted, eventually vacuumed
-- ACID violation appearance: Durability intentionally bypassed

-- NOP 4: Null-Effect UPSERT
-- ON CONFLICT clause ensures no insertion or update occurs
INSERT INTO config (key, value)
VALUES ('test_key', 'test_value')
ON CONFLICT (key) DO NOTHING;
-- If key exists: No operation (pure NOP)
-- If key doesn't exist: Would insert (conditional NOP)
-- Locks: Unique index lock acquired, released without modification
-- Query plan: Index Scan → Conflict Check → Skip

-- NOP 5: CTE with No Side Effects
-- Common Table Expression computed but not materialized
WITH calculated AS (
    SELECT user_id, COUNT(*) as activity_count
    FROM user_actions
    WHERE created_at < '1970-01-01'  -- Always false
    GROUP BY user_id
)
SELECT * FROM calculated;
-- Query plan: CTE Scan → Aggregate (0 rows) → Result (empty)
-- Index usage: Timestamp index consulted, no blocks read
-- Statistics impact: None (early termination)

-- NOP 6: Advisory Lock Dance
-- Acquires and releases application-level locks
SELECT pg_advisory_lock(12345);
SELECT pg_advisory_unlock(12345);
-- No data modification, pure lock protocol exercise
-- Transaction log: No entries (advisory locks not logged)
-- Useful for: Simulating lock contention without state change

-- NOP 7: Parameterized No-Op with EXPLAIN
-- Query planning overhead without execution
EXPLAIN (ANALYZE false, COSTS false)
SELECT * FROM users WHERE false;
-- Only planning cost, no execution
-- Catalog access: Yes (table metadata)
-- Index impact: None (query not executed)

-- ----------------------------------------------------------------------------
-- MYSQL EXAMPLES
-- ----------------------------------------------------------------------------

-- NOP 8: Empty UPDATE with WHERE 1=0 (MySQL)
START TRANSACTION;
UPDATE users SET email = email WHERE 1 = 0;
-- InnoDB: No undo log entries despite UPDATE statement
-- Binary log: Statement recorded (replication overhead)
-- Locks: Intention locks on table, no row locks
COMMIT;

-- NOP 9: SELECT...FOR UPDATE on Empty Set (MySQL)
START TRANSACTION;
SELECT * FROM users WHERE id < 0 FOR UPDATE;
-- Locks: Gap locks acquired in index (even for no rows)
-- MVCC: Read view created, no rows returned
-- Binary log: No entry (read-only operation)
COMMIT;

-- NOP 10: REPLACE with Duplicate Values (MySQL)
-- If row exists with same primary key, deletes then inserts identical data
REPLACE INTO settings (id, setting_name, setting_value)
SELECT id, setting_name, setting_value FROM settings WHERE id = 1;
-- Result: Net zero change, but DELETE + INSERT in binary log
-- Auto-increment: May advance if REPLACE succeeds
-- Locks: Exclusive row lock → delete → insert → release
-- Observability: Last_Insert_ID may change despite no net effect

-- NOP 11: Optimistic Locking Failure Simulation
UPDATE versioned_data
SET data = 'new_value', version = version + 1
WHERE id = 123 AND version = 999;
-- If version mismatch: 0 rows updated (optimistic lock failure)
-- Application-level CNO: Attempt made, nothing changed
-- Index: Version checked via index scan, no update

-- ----------------------------------------------------------------------------
-- SQLITE EXAMPLES
-- ----------------------------------------------------------------------------

-- NOP 12: Vacuous DELETE (SQLite)
DELETE FROM users WHERE rowid IN (SELECT rowid FROM users WHERE 1=0);
-- Query plan: Scan subquery → empty set → delete 0 rows
-- Journal: Rollback journal created, no page modifications
-- Locks: RESERVED lock acquired, released

-- NOP 13: Self-Referential UPDATE (SQLite)
BEGIN IMMEDIATE;
UPDATE products SET price = price;
-- Every row "updated" to same value
-- Journal: Full page copies written to rollback journal
-- Result: Database file unchanged after commit
-- ACID: Full transaction overhead for zero semantic change
COMMIT;

-- NOP 14: INSERT OR IGNORE of Existing Row (SQLite)
INSERT OR IGNORE INTO unique_items (id, name)
VALUES (1, 'existing_item');
-- If id=1 exists: Constraint violation → IGNORE → no operation
-- Journal: No entries (early abort on constraint check)
-- Locks: Brief EXCLUSIVE lock for constraint check

-- NOP 15: Null Change with TRIGGER awareness
-- Even if triggers exist, no net effect if condition fails
UPDATE monitored_table SET status = 'changed' WHERE 1 = 0;
-- Triggers: Not fired (0 rows affected)
-- Shadow tables: Not updated
-- Audit trail: No entries

-- ----------------------------------------------------------------------------
-- CROSS-DATABASE: ACID PROPERTY ANALYSIS
-- ----------------------------------------------------------------------------

-- NOP 16: Read-Only Transaction (All Databases)
BEGIN;
SELECT COUNT(*) FROM large_table WHERE random_column = 'nonexistent_value';
-- Atomicity: Single operation, atomic by definition
-- Consistency: No constraints violated (read-only)
-- Isolation: Snapshot created (MVCC overhead)
-- Durability: N/A (no writes)
-- Result: Full transaction machinery for zero state change
COMMIT;

-- NOP 17: Constraint Validation Without Modification
-- Check constraint evaluated, no row changes
BEGIN;
INSERT INTO validated_table (amount) VALUES (100)
ON CONFLICT DO NOTHING;  -- PostgreSQL
-- Constraint check: amount > 0 evaluated
-- If conflict: No insert, no update
-- If no conflict: Insert occurs (conditional NOP)
ROLLBACK;  -- Ensures NOP regardless

-- ----------------------------------------------------------------------------
-- PERFORMANCE OVERHEAD ANALYSIS
-- ----------------------------------------------------------------------------

/*
TRANSACTION LOG OVERHEAD:
- PostgreSQL WAL: BEGIN/COMMIT records (24 bytes minimum)
- MySQL binlog: Statement or row format records
- SQLite journal: Database page backups even for no-change transactions

LOCK OVERHEAD:
- Table-level metadata locks (all databases)
- Row-level intention locks (InnoDB, PostgreSQL)
- Gap locks for range conditions (MySQL InnoDB)
- Advisory locks (PostgreSQL)

QUERY PLAN COSTS:
- Parser overhead: SQL text → parse tree
- Planner overhead: Statistics consultation, cost estimation
- Executor overhead: Minimal (early termination on false conditions)

INDEX IMPACT:
- Index consulted for WHERE clauses
- Statistics updated on table access (minor increments)
- No index modifications (no data changed)
- Buffer pool pollution (index pages cached)

MVCC OVERHEAD:
- Transaction ID allocation
- Snapshot creation (read-consistent view)
- Visibility checks (tuple versioning)
- Vacuum pressure (aborted transactions)

OBSERVABILITY:
- Query logs: Statement recorded
- Performance schema: Execution time tracked
- Statistics tables: Access counters incremented
- Monitoring: Resource consumption (CPU, I/O) without data change
*/

-- ----------------------------------------------------------------------------
-- CNO OPERATIONAL CHARACTERISTICS
-- ----------------------------------------------------------------------------

/*
A database-level CNO (Computer Network Operations / No-Op) exhibits:

1. EXECUTION SUCCESS: Query completes without error
2. ZERO DATA MUTATION: Database state identical before and after
3. RESOURCE CONSUMPTION: CPU, memory, I/O utilized
4. OBSERVABLE TRACES: Logs, statistics, metrics record activity
5. LOCK PROTOCOL: Concurrency control mechanisms engaged
6. TRANSACTION OVERHEAD: ACID machinery invoked

Use cases:
- Probing database responsiveness without state change
- Testing lock behavior and contention
- Warming up query plan cache
- Simulating load without data modification
- Security research: Observing behavior without mutation
- Benchmarking transaction overhead

Anti-patterns:
- Using as "keep-alive" (better mechanisms exist)
- Performance testing (doesn't reflect real workload)
- Assuming zero cost (resource overhead is real)
*/

-- ============================================================================
-- END NOP.SQL
-- ============================================================================

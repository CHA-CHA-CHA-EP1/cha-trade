# POST /internal/accounts/:id/settle

## Request Body

```json
{
  "from_account_id": "uuid",
  "to_account_id": "uuid",
  "amount": 30000,
  "order_id": "uuid"
}
```

## Response Body

```json
{
  "success": true
}
```

## Business Logic

### Validate Request Body
1. `from_account_id` must not be empty and must be a valid UUID
2. `to_account_id` must not be empty and must be a valid UUID
3. `from_account_id` and `to_account_id` must not be the same account
4. `amount` must not be empty and must be a number greater than 0
5. `order_id` must not be empty and must be a valid UUID

### Steps
1. Begin database transaction
2. Lock both rows with `SELECT ... FOR UPDATE` ordered by UUID to prevent deadlocks
3. Check that `from_account_id` exists → if not found, return 404
4. Check that `to_account_id` exists → if not found, return 404
5. Check that `hold_balance` of `from_account >= amount` → if insufficient, return 422
6. Update `from_account`: `hold_balance = hold_balance - amount`
7. Update `to_account`: `available_balance = available_balance + amount`
8. Insert 2 rows in the `transactions` table
   - row 1 (buyer/from): `transaction_type = 'settle'`, `direction = 'debit'`, `order_id`
   - row 2 (seller/to): `transaction_type = 'settle'`, `direction = 'credit'`, `order_id`
9. Commit transaction — steps 6, 7, 8 must all succeed together; if any step fails, rollback everything
10. Return `{ success: true }`

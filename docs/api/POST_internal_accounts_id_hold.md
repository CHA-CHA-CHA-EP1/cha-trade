# POST /internal/accounts/:id/hold

## Request Body

```json
{
  "amount": 30000,
  "order_id": "uuid"
}
```

## Response Body

```json
{
  "success": true,
  "available_balance": 70000,
  "hold_balance": 30000
}
```

## Business Logic

### Validate Path Params
1. `:id` must be a valid UUID

### Validate Request Body
1. `amount` must not be empty and must be a number greater than 0
2. `order_id` must not be empty and must be a valid UUID

### Steps
1. Query the `accounts` table by `id` → if not found, return 404
2. Begin database transaction
3. Lock the row with `SELECT ... FOR UPDATE` to prevent race conditions
4. Check that `available_balance >= amount` → if insufficient, return 422
5. Record `balance_before = current available_balance`
6. Update `available_balance = available_balance - amount` and `hold_balance = hold_balance + amount`
7. Insert a row in the `transactions` table
   - `transaction_type = 'hold'`
   - `direction = 'debit'`
   - `order_id`
   - `balance_before`, `balance_after`
8. Commit transaction
9. Return updated balance

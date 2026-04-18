# POST /internal/accounts/:id/release

## Curl

```bash
curl -X POST http://localhost:8080/internal/accounts/{id}/release \
  -H "Content-Type: application/json" \
  -d '{"amount": 30000, "order_id": "uuid"}'
```

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
  "available_balance": 100000,
  "hold_balance": 0
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
3. Lock the row with `SELECT ... FOR UPDATE`
4. Check that `hold_balance >= amount` → if insufficient, return 422
5. Record `balance_before = current hold_balance`
6. Update `hold_balance = hold_balance - amount` and `available_balance = available_balance + amount`
7. Insert a row in the `transactions` table
   - `transaction_type = 'release'`
   - `direction = 'credit'`
   - `order_id`
   - `balance_before`, `balance_after`
8. Commit transaction
9. Return updated balance

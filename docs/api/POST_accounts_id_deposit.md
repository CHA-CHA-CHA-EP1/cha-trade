# POST /accounts/:id/deposit

## Request Body

```json
{
  "amount": 50000
}
```

## Response Body

```json
{
  "account_id": "uuid",
  "available_balance": 150000,
  "hold_balance": 0
}
```

## Business Logic

### Validate Path Params
1. `:id` must be a valid UUID

### Validate Request Body
1. `amount` must not be empty
2. `amount` must be a number greater than 0

### Steps
1. Query the `accounts` table by `id` → if not found, return 404
2. Begin database transaction
3. Lock the row with `SELECT ... FOR UPDATE` to prevent race conditions
4. Record `balance_before = current available_balance`
5. Update `available_balance = available_balance + amount`
6. Insert a row in the `transactions` table
   - `transaction_type = 'deposit'`
   - `direction = 'credit'`
   - `balance_before`, `balance_after`
7. Commit transaction
8. Return updated balance

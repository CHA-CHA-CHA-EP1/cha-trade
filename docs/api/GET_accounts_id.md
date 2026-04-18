# GET /accounts/:id

## Response Body

```json
{
  "id": "uuid",
  "user_id": "uuid",
  "available_balance": 100000,
  "hold_balance": 0,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

## Business Logic

### Validate Path Params
1. `:id` must be a valid UUID

### Steps
1. Query the `accounts` table by `id`
2. If not found, return 404
3. Return the account data
